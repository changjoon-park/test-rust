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

fn main() {
    let books = vec![
        Book::Fiction {
            title: String::from("The Hobbit"),
            description: Some(String::from("A fantasy novel by J.R.R")),
        },
        Book::NonFiction {
            title: String::from("A Brief History of Time"),
            description: None,
        },
        Book::Fiction {
            title: String::from("1984"),
            description: Some(String::from("A dystopian novel by George Orwen")),
        },
    ];

    for book in &books {
        display_book_details(book);
    }
}

fn display_book_details(book: &Book) {
    match book {
        Book::Fiction { title, description } => {
            println!("Fiction Book: {title}");
            if let Some(desc) = description {
                println!("Decription: {desc}");
            } else {
                println!("Description: No Description available");
            }
        }
        Book::NonFiction { title, description } => {
            println!("Non Fiction Book: {title}");
            if let Some(desc) = description {
                println!("Description: {desc}");
            } else {
                println!("Description: No desciption availabe");
            }
        }
    }
}
