fn most_frequent_word(text: &str) -> (String, usize)
{
    let mut top_count = 0;
    let mut top_word = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 0..words.len()
    {
        let mut count = 0;          
        for j in 0..words.len()
        {
            if words[i] == words[j]
            {
                count += 1;
            }
        }

        if count > top_count
        {
            top_count = count;
            top_word = words[i].to_string(); 
        }
    }
    return (top_word, top_count)
}


fn main() 
{
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}