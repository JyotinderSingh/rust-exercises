mod book;
mod library;
mod person;

use book::Book;
use library::Library;
use person::Person;

fn get_person_details(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() -> Result<(), String> {
    // Borrow a Person.
    let alice = Person {
        name: "Alice".to_string(),
        age: 25,
    };
    get_person_details(&alice);

    // Create a Library struct.
    let mut library = Library {
        books: vec![
            Book {
                title: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik and Carol Nichols".to_string(),
                is_available: true,
                borrower: None,
            },
            Book {
                title: "Programming Rust: Fast, Safe Systems Development".to_string(),
                author: " Jim Blandy, Jason Orendorff, Leonora F.S. Tindall".to_string(),
                is_available: true,
                borrower: None,
            },
        ],
    };

    // Check out a book.
    library.check_out_book(String::from("The Rust Programming Language"), &alice)?;

    // List all available books.
    library.list_available_books();

    // List all checked out books.
    library.list_checked_out_books();

    // Return the book.
    library.return_book(String::from("The Rust Programming Language"), &alice)?;

    return Ok(());
}
