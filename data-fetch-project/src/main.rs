use serde::Deserialize; 
// For the 10 second delay and current time
use std::thread;
use std::time::Duration;
use chrono::Local;
use std::io::Write;
use ureq::serde_json;
use crate::serde_json::Value;     
// To create and open files       
use std::fs::OpenOptions;    

const SP500_URL: &str = "https://stooq.pl/q/l/?s=%5Espx&f=sd2t2ohlcv&h&e=json";
const BTC_URL: &str = "https://api.coinbase.com/v2/prices/BTC-USD/spot";
const ETH_URL: &str = "https://api.coinbase.com/v2/prices/ETH-USD/spot";

// Abstract
trait Pricing
{
    fn get_name(&self) -> &str;
    fn fetch_price(&mut self) -> f64;
    fn save_to_file(&self); // one file for each currency
}

// Structs
#[derive(Debug, Deserialize)]
struct Bitcoin
{ price: f64, }

#[derive(Debug, Deserialize)]
struct Ethereum
{ price: f64, }

#[derive(Debug, Deserialize)]
struct SP500
{ price: f64, }


// Timestamps
fn get_time() -> String 
{
    let time = Local::now();
    return time.format("%Y-%m-%d %H:%M:%S").to_string();
}

// Implements 
impl Pricing for Bitcoin
{
    fn get_name(&self) -> &str
    { return "BTC"; }

    fn fetch_price(&mut self) -> f64
    {
        match ureq::get(BTC_URL).call()
        {
            Ok(response) =>
            {
                if response.status() == 200
                {
                    let v: Value = ureq::get(BTC_URL).call().unwrap().into_json().unwrap();
                    let amount = v["data"]["amount"].as_str().unwrap().parse::<f64>().unwrap();
                    self.price = amount;
                    return amount;
                }
                else
                {
                    eprintln!("HTTP status: {}", response.status());
                    
                }
            },
            Err(e) => 
            {
                let error_details = format!("Request failed: {}", e);
                eprintln!("{}", error_details);
            }
        }
        return 0.0;
    }

    fn save_to_file(&self)
    {
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("bitcoin.csv")
        .unwrap();
        let time = get_time();
        writeln!(file, "Time: {} | Price: {}", time, self.price).unwrap();
    }
}

impl Pricing for Ethereum
{
    fn get_name(&self) -> &str
    { return "ETH"; }

    fn fetch_price(&mut self) -> f64
    {
        match ureq::get(ETH_URL).call()
        {
            Ok(response) =>
            {
                if response.status() == 200
                {
                    let v: Value = ureq::get(ETH_URL).call().unwrap().into_json().unwrap();
                    let amount = v["data"]["amount"].as_str().unwrap().parse::<f64>().unwrap();
                    self.price = amount;
                    return amount;
                }
                else
                {
                   eprintln!("HTTP status: {}", response.status());
                }
            },
            Err(e) => 
            {
                  let error_details = format!("Request failed: {}", e);
                eprintln!("{}", error_details);
            }
        }
        return 0.0;
    }

    fn save_to_file(&self)
    {
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ethereum.csv")
        .unwrap();
        let time = get_time();
        writeln!(file, "Time: {} | Price: {}", time, self.price).unwrap();
    }
}

impl Pricing for SP500
{
    fn get_name(&self) -> &str
    { return "S&P500"; }

    fn fetch_price(&mut self) -> f64
    {
        match ureq::get(SP500_URL).call()
        {
            Ok(response) =>
            {
                if response.status() == 200
                {
                    let v: Value = ureq::get(SP500_URL).call().unwrap().into_json().unwrap();
                    let close = v["symbols"][0]["close"].as_f64().unwrap();
                    self.price = close;
                    return close;
                }
                else
                {
                   eprintln!("HTTP status: {}", response.status());
                }
            },
            Err(e) => 
            {
                let error_details = format!("Request failed: {}", e);
                eprintln!("{}", error_details);
            }
        }
        return 0.0;
    }

    fn save_to_file(&self)
    {
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sp500.csv")
        .unwrap();
         let time = get_time();
        writeln!(file, "Time: {} | Price: {}", time, self.price).unwrap();
    }
}

fn main()
{
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        Box::new(Ethereum { price: 0.0 }),
        Box::new(SP500 { price: 0.0 }),
    ];

    loop
    {
        for asset in assets.iter_mut()
        { 
            let price = asset.fetch_price();
            asset.save_to_file();
            println!("{}: {}", asset.get_name(), price);
            println!("---------------------------------------------");
        }
        thread::sleep(Duration::from_secs(10));
    }
}