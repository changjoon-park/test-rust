use dialoguer::{theme::ColorfulTheme, Input, Select};

struct BookDetails {
    title: String,
    description: Option<String>,
}

enum Book {
    Fiction(BookDetails),
    NonFiction(BookDetails),
}

fn display_book_details(book: &Book) {
    match book {
        Book::Fiction(details) => {
            println!("Fiction Book: {}", details.title);
            match &details.description {
                Some(desc) => println!("Description: {}", desc),
                None => println!("Description: No description available"),
            }
        }
        Book::NonFiction(details) => {
            println!("Non-Fiction Book: {}", details.title);
            match &details.description {
                Some(desc) => println!("Description: {}", desc),
                None => println!("Description: No description available"),
            }
        }
    }
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

        let items = &["Fiction", "NonFiction"];

        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select Category: ")
            .default(0)
            .items(&items[..])
            .interact()
            .unwrap();

        let details = BookDetails {
            title,
            description: Some(description),
        };

        if items[selection] == "Fiction" {
            books.push(Book::Fiction(details));
        } else {
            books.push(Book::NonFiction(details));
        }
    }

    for book in &books {
        display_book_details(book);
    }
}
