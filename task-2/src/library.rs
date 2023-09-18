use crate::book::Book;
use crate::person::Person;

pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn check_out_book(&mut self, title: String, borrower: &Person) -> Result<(), String> {
        // Find the book by title.
        let book = self.find_book(title)?;

        // Check if the book is available.
        if !book.is_available {
            return Err(String::from("Book is not available."));
        }

        book.is_available = false;
        book.borrower = Some(borrower.name.clone());

        return Ok(());
    }

    pub fn return_book(&mut self, title: String, borrower: &Person) -> Result<(), String> {
        let book = self.find_book(title)?;
        // Check if the book is available.
        if book.is_available || book.borrower.as_ref().unwrap().as_ref() != borrower.name {
            return Err(String::from("Book is not issued."));
        }

        book.borrower = None;
        book.is_available = true;

        return Ok(());
    }

    pub fn list_available_books(&self) {
        self.books
            .iter()
            .filter(|book| book.is_available)
            .for_each(|book| println!("{} by {}", book.title, book.author));
    }

    pub fn list_checked_out_books(&self) {
        self.books
            .iter()
            .filter(|book| !book.is_available)
            .for_each(|book| {
                println!(
                    "{} by {} is borrowed by {}",
                    book.title,
                    book.author,
                    book.borrower.as_ref().unwrap()
                )
            });
    }

    fn find_book(&mut self, title: String) -> Result<&mut Book, String> {
        // Find the book by title.
        let book = self.books.iter_mut().find(|book| book.title == title);

        if book.is_none() {
            return Err(format!("Book {} not found.", title));
        }

        // Return the book.
        return Ok(book.unwrap());
    }
}
