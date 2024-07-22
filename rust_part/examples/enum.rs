use dialoguer::{theme::ColorfulTheme, Input, Select};

enum Book {
    Fiction(BookDetail),
    NonFiction(BookDetail),
}

enum IpAddr {
    V4(String),
    V6(String),
}

impl Book {
    fn display_detail(&self) {
        match self {
            Book::Fiction(detail) => {
                println!("Fiction Book: {}", detail.title);
                match &detail.description {
                    Some(desc) => println!("Description: {}", desc),
                    None => println!("No Description"),
                }
            }
            Book::NonFiction(detail) => {
                println!("Non-Fiction Book: {}", detail.title);
                match &detail.description {
                    Some(desc) => println!("Description: {}", desc),
                    None => println!("No Description"),
                }
            }
        }
    }
}

struct BookDetail {
    title: String,
    description: Option<String>,
}

fn main() {
    let mut books = Vec::new();

    loop {
        let title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Title: ")
            .interact_text()
            .unwrap();

        if title.to_lowercase() == "q" {
            break;
        }

        let description: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Description: ")
            .interact_text()
            .unwrap();

        let book_detail = BookDetail {
            title,
            description: Some(description),
        };

        let items = &["Fiction", "NonFiction"];

        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select Category")
            .default(0)
            .items(&items[..])
            .interact()
            .unwrap();

        if items[selection] == "Fiction" {
            books.push(Book::Fiction(book_detail));
        } else if items[selection] == "NonFiction" {
            books.push(Book::NonFiction(book_detail));
        }
    }

    for book in &books {
        book.display_detail();
    }
}
