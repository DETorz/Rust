#[derive(Debug, PartialEq)]
enum BookStatus {
    Available,
    CheckedOut(i32),
    BeingRead,
    InRepair(String),
    Lost,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    status: BookStatus,
}

impl Book {
    fn new(title: String, author: String) -> Self {
        Book{
           title,
           author,
           status: BookStatus::Available, 
        }        
    }

    fn check_out(&mut self, days: i32) {
            self.status = BookStatus::CheckedOut(days);
    }

    fn return_book(&mut self) {
        self.status = BookStatus::Available;
    }

    fn send_for_repair(&mut self, notes: String) {
        self.status = BookStatus::InRepair(notes);
    }

    fn mark_as_being_read(&mut self) {
        self.status =BookStatus::BeingRead;
    }

    fn report_lost(&mut self) {
        self.status = BookStatus::Lost;
    }

    fn display_status(&self) -> String {
        match &self.status {        // match will if-else check which status(es) are true and use the => operator to run a simple instruction
            BookStatus::Available => format!("{} is available for borrowing.", self.title),
            // keep covering cases
            BookStatus::CheckedOut(days) => format!("{} is checked out. Days until due: {}", self.title, days),
            BookStatus::BeingRead => format!("{} is currently being read.", self.title),
            BookStatus::InRepair(notes) => format!("{} is in repair. Notes: {}", self.title, notes),
            BookStatus::Lost => format!("{} has been reported lost.", self.title)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_out_and_return_book() {
        let mut book = Book::new("Rust Programming".to_string(), "John Doe".to_string());

        book.check_out(14);
        assert_eq!(book.status, BookStatus::CheckedOut(14));

        book.return_book();
        assert_eq!(book.status, BookStatus::Available);
    }

    #[test]
    fn send_book_for_repair() {
        let mut book = Book::new("Learning Rust".to_string(), "Jane Smith".to_string());

        book.send_for_repair("Broken spine".to_string());
        match book.status {
            BookStatus::InRepair(ref notes) if notes == "Broken spine" => (),
            _ => panic!("Book was not properly sent for repair."),
        }
    }

    #[test]
    fn mark_book_as_being_read() {
        let mut book = Book::new("Rust in Action".to_string(), "Tim McNamara".to_string());

        book.mark_as_being_read();
        assert_eq!(book.status, BookStatus::BeingRead);
    }

    #[test]
    fn report_book_as_lost() {
        let mut book = Book::new("Programming Rust".to_string(), "Jim Blandy".to_string());

        book.report_lost();
        assert_eq!(book.status, BookStatus::Lost);
    }

    #[test]
    fn display_status_for_each_case() {
        let available_book = Book::new("Rust Programming".to_string(), "John Doe".to_string());
        assert_eq!(available_book.display_status(), "Rust Programming is available for borrowing.");

        let mut checked_out_book = Book::new("Learning Rust".to_string(), "Jane Smith".to_string());
        checked_out_book.check_out(14);
        assert!(checked_out_book.display_status().contains("is checked out. Days until due: 14"));

        let mut being_read_book = Book::new("Rust in Action".to_string(), "Tim McNamara".to_string());
        being_read_book.mark_as_being_read();
        assert_eq!(being_read_book.display_status(), "Rust in Action is currently being read.");

        let mut in_repair_book = Book::new("Programming Rust".to_string(), "Jim Blandy".to_string());
        in_repair_book.send_for_repair("Broken spine".to_string());
        assert!(in_repair_book.display_status().contains("is in repair. Notes: Broken spine"));

        let lost_book = Book {
            title: "Zero To Production In Rust".to_string(),
            author: "Luca Palmieri".to_string(),
            status: BookStatus::Lost,
        };
        assert_eq!(lost_book.display_status(), "Zero To Production In Rust has been reported lost.");
    }
}