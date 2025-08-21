use std::time::{ Duration, Instant, SystemTime, UNIX_EPOCH, };
use std::fs;
use std::sync::mpsc;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use ureq::serde_json::{self, json, Value, };

#[derive(Debug, PartialEq)]
enum CheckError
{
    Timeout,
    Transport(String),
}

#[derive(Debug)]
struct Config
{
    workers_t: usize,
    timeout_secs: u32,
    retries: usize,
}

impl Config
{
    const DEFAULT_WORKERS: usize = 50;
    const DEFAULT_TIMEOUT_SECS: u32 = 5;
    const DEFAULT_RETRIES: usize = 0;

    fn new() -> Self
    {
        Self
        {
            workers_t: Self::DEFAULT_WORKERS,
            timeout_secs: Self::DEFAULT_TIMEOUT_SECS,
            retries: Self::DEFAULT_RETRIES,
        }
    }
}

// There is probably a better way to do this, like args in the terminal
impl Default for Config
{
    fn default() -> Self { Self::new() }
}

#[derive(Debug)]
struct WebsiteStatus
{
    url: String,
    status: Result<u16, CheckError>,
    response_time: Duration,
    timestamp: SystemTime, 
}

fn collect_urls_from_txt(path: &str) -> Result<Vec<String>, String>
{
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(content
    .lines()
    .filter(|l| !l.trim().is_empty())
    .map(|l| l.trim().to_string())
    .collect()
    )
}

fn status_to_json_helper(ws: &WebsiteStatus) -> Value 
{
    let status_string = match &ws.status 
    {
        Ok(code) => code.to_string(),
        Err(CheckError::Timeout) => "Timed out".to_string(),
        Err(CheckError::Transport(s)) => s.clone(),
    };
    let response_time = format!("{}ms", ws.response_time.as_millis());
    let timestamp = ws
        .timestamp
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    json!({
        "url": &ws.url,
        "status": status_string,
        "response_time": response_time,
        "timestamp": timestamp,
    })
}

fn status_to_json(i_file: &str, o_file: &str, workers_t: usize, timeout: Duration, retries: usize) -> Result<(), String> 
{
    let urls = collect_urls_from_txt(i_file)?;

    // Shared work queue 
    let queue = Arc::new(Mutex::new(VecDeque::from(urls)));
    let (result_transmit, result_recieve) = mpsc::channel::<Value>();

    // Spawn up to workers_t threads (but not more than number of URLs, and at least 1)
    let initial_len = { queue.lock().unwrap().len() };
    let spawn_count = workers_t.min(initial_len.max(1));

    let mut handles = Vec::with_capacity(spawn_count);
    for _ in 0..spawn_count 
    {
        let q = Arc::clone(&queue);
        let transmit = result_transmit.clone();
        let timeout_copy = timeout; 
        let retries_copy = retries;

        handles.push(thread::spawn(move ||  
        {
            loop 
            {
                let maybe_url = 
                {
                    let mut guard = q.lock().unwrap();
                    guard.pop_front()
                };

                match maybe_url 
                {
                    // `Some` acts like an enum but the difference is we have work for to do if there is a url
                    Some(url) => 
                    {
                        let ws = check_website_with_retries(&url, timeout_copy, retries_copy);
                        let new_row = status_to_json_helper(&ws);
                        let _ = transmit.send(new_row);
                    }
                    // `None` if there is no url given, we just break the loop
                    None => break, 
                }
            }
        }));
    }
    drop(result_transmit); // close main sender so iterator ends after workers finish

    let mut rows: Vec<Value> = Vec::new();
    for row in result_recieve.iter() 
    {
        rows.push(row);
    }

    for handle in handles 
    {
        let _ = handle.join();
    }

    let last = fs::File::create(o_file).map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(last, &rows).map_err(|e| e.to_string())?;

    return Ok(());
}

fn check_website_with_retries(url: &str, timeout: Duration, retries: usize) -> WebsiteStatus
{
    let mut total_elapsed = Duration::from_secs(0);

    for attempt in 0..=retries
    {
        let start = Instant::now();
        let ws = check_website(url, timeout);
        let elapsed = start.elapsed();
        total_elapsed += elapsed;

        match ws.status
        {
            Ok(_) => 
            {
                return WebsiteStatus{response_time: total_elapsed, ..ws};
            }
            Err(CheckError::Timeout) => 
            {
                return WebsiteStatus {response_time: total_elapsed, ..ws};
            }
            Err(CheckError::Transport(_)) => 
            {
                if attempt < retries
                {
                    let delay = 50 * (1 << attempt.min(6));
                    thread::sleep(Duration::from_millis(delay));
                    continue;
                }
                else
                {
                    return WebsiteStatus {response_time: total_elapsed, ..ws};
                }
            }
        }
    }
    return check_website(url, timeout);
}

fn check_website(url: &str, timeout: Duration) -> WebsiteStatus
{
    let agent = ureq::AgentBuilder::new()
    .timeout_connect(timeout)
    .timeout_read(timeout)
    .timeout_write(timeout)
    .build();

    let start = Instant::now(); 
    let result = agent
    .get(url)
    .set("User-agent", "ExampleHeader/0.1")
    .call();
    let elapsed = start.elapsed();
    let status = match result
    {
        Ok(response) => Ok(response.status()),
        Err(ureq::Error::Status(status_code, _)) => Ok(status_code),
        Err(e) =>
        {
            if elapsed >= timeout 
            {
                Err(CheckError::Timeout)
            }
            else
            {
                Err(CheckError::Transport(e.to_string()))
            }
        }
    };

        return WebsiteStatus{
        url: url.to_string(),
        status,
        response_time: elapsed,
        timestamp: SystemTime::now(),
    };
}

fn main()
{
    let con = Config::default();
    const INPUT: &str = "website_list.txt";
    const OUTPUT: &str = "status_report.json";
    println!("Reading URL list from {}", INPUT);
    match status_to_json(INPUT, OUTPUT, con.workers_t, Duration::from_secs(con.timeout_secs.into()), con.retries)
    {
        Ok(()) => println!("Wrote report to {}", OUTPUT),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/*
 - Unit tests for core functionality
1) Integration tests with mock HTTP server
2) Performance tests with multiple concurrent requests
3) Error handling tests
*/
#[cfg(test)]
mod tests 
{
    use super::*; // Include all previous imports
    use std::net::TcpListener;
    use std::io::{Read, Write}; 

    fn mock_http_server(delay_ms: u64, status: u16, request_capacity: usize,) -> (String, thread::JoinHandle<()>) 
    {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Bind failed");
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        // Move accepting/serving into a background thread so tests don't block
        let handle = thread::spawn(move || 
        {
            for _ in 0..request_capacity 
            {
                match listener.accept() 
                {
                    Ok((mut stream, _)) => 
                    {
                        // Read and discard the request (ensures headers arrive)
                        let mut buffer = [0; 1024];
                        let _ = stream.read(&mut buffer);
                        // Add delay for more tests
                        thread::sleep(Duration::from_millis(delay_ms));
                        let (reason, body) = match status 
                        {
                            200 => ("OK", ""), 
                            404 => ("URL not found", ""),
                            500 => ("Internal Server Error", ""),
                            _ => ("", ""),
                        };
                        let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", status, reason, body.len(), body);
                        let _ = stream.write_all(response.as_bytes());
                    }
                    Err(e) => 
                    {
                        eprintln!("{e}");
                        break;
                    }
                }
            }
        });
       return (url, handle);
    }

    #[test]
    fn mock_test_http_codes()
    {
        let cases: &[u16] = &[404, 500, 503, 200];
        for &code in cases
        {
            let (url, handle) = mock_http_server(0, code, 1);
            let web_status = check_website(&url, Duration::from_secs(5));

            match web_status.status
            {
                Ok(s) => assert_eq!(s, code, "expected {}, got {:?}", code, web_status.status),
                Err(e) => panic!("expected HTTP status {}, got transport error: {:?}", code, e),
            }
            handle.join().unwrap();
        }
    }

    #[test]
    fn mock_test_web_timeout()
    {
        let (url, handle) = mock_http_server(5100, 200, 1); // 5.1 seconds
        let timeout = Duration::from_secs(5); 
        let web_status = check_website(&url, timeout);
        assert!(matches!(web_status.status, Err(CheckError::Timeout)));
        assert!(web_status.response_time >= timeout, "response_time {:?} was below timeout {:?}", web_status.response_time, timeout);

        handle.join().unwrap();
    }

    #[test]
    fn mock_test_concurrency_performance()
    {
        const MAX: usize = 50;
        let server_delay = Duration::from_millis(100);
        let client_timeout = Duration::from_secs(5);
        let mut servers: Vec<(String, thread::JoinHandle<()>)> = Vec::with_capacity(MAX);
        for _ in 0..MAX
        {
            let(url, handle) = mock_http_server(server_delay.as_millis() as u64, 200, 1);
            servers.push((url, handle));
        }
        let start = Instant::now();
        let mut client_handles: Vec<thread::JoinHandle<WebsiteStatus>> = Vec::with_capacity(MAX);
        for(url, _) in &servers
        {
            let url = url.clone();
            client_handles.push(thread::spawn(move || 
            {
                check_website(&url, client_timeout)
            }));
        }
        // Get result
        for handle in client_handles
        {
            let web_status = handle.join().expect("iteration: {} client thread panicked");
            assert!(matches!(web_status.status, Ok(200)), "Expected 200, got {:?} (url: {})", web_status.status, web_status.url);
        }
        let elapsed = start.elapsed();
        // Clean up threads
        for(_, handle) in servers
        {
            handle.join().unwrap();
        }

        let upper = server_delay * 10; // 100ms * 10 = 1 second
        assert!(elapsed < upper, "Concurrent had a slow performance: {:?} (expected < {:?})", elapsed, upper);
    }
}
