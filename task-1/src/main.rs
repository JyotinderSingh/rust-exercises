struct Person {
    name: String,
    age: u8,
}

struct Book {
    title: String,
    author: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}

fn main() {
    let person = Person {
        name: "Jyotinder Singh".to_string(),
        age: 30,
    };

    assert_eq!(person.name, "Jyotinder Singh");
    assert_eq!(person.age, 30);

    let mut library = Library { books: vec![] };

    let book = Book {
        title: "Lorem Ipsum".to_string(),
        author: "Author Name".to_string(),
        is_available: true,
    };

    library.books.push(book);

    assert_eq!(library.books.len(), 1);
    assert_eq!(library.books[0].title, "Lorem Ipsum");
    assert_eq!(library.books[0].author, "Author Name");
    assert_eq!(library.books[0].is_available, true);
}
