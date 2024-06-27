enum Book {
    Fiction {
        title: String,
        description: Option<String>,
    },
    NonFiction {
        title: String,
        description: Option<String>,
    },
}

fn display_book_details(book: &Book) {
    match book {
        Book::Fiction { title, description } => {
            println!("Fiction Book: {}", title);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            } else {
                println!("Description: No description available.");
            }
        }
        Book::NonFiction { title, description } => {
            println!("Non-Fiction Book: {}", title);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            } else {
                println!("Description: No description available.");
            }
        }
    }
}

fn main() {
    let book1 = Book::Fiction {
        title: String::from("The Hobbit"),
        description: Some(String::from("A fantasy novel by J.R.R. Tolkien.")),
    };

    let book2 = Book::NonFiction {
        title: String::from("A Brief History of Time"),
        description: None,
    };

    let book3 = Book::Fiction {
        title: String::from("1984"),
        description: Some(String::from("A dystopian novel by George Orwell.")),
    };

    let books = vec![book1, book2, book3];

    for book in books {
        display_book_details(&book);
    }
}
