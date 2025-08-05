use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book 
{
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) 
{
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();
    for book in books
    {
        writeln!(file, "{}, {}, {}", book.title, book.author, book.year)
        .expect("File could not be opened");
    }
}

fn load_books(filename: &str) -> Vec<Book> 
{
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let r = BufReader::new(file);
    let mut books = Vec::new();
    for line in r.lines()
    {
        let line = line
        .expect("Had a problem reading the line");
        let part: Vec<_> = line.split(',').map(str::trim).collect(); // does some magic, removes the whitespace, then splices the strings based on the commas
        let title = part[0].to_string();
        let author = part[1].to_string();
        let year: u16 = part[2].parse()
        .expect("Could not parse the year");
        books.push(Book {title, author, year} );

    }

    return books;
}

fn main() 
{
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Lightning Thief".to_string(), author: "Rick Riordan".to_string(), year: 2006 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");

    for book in loaded_books 
    {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}