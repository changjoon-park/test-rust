use std::mem::size_of_val;

struct Book {
    id: u32,
    title: String,
    authors: Vec<String>,
}

fn main() {
    let book1 = Book {
        id: 1,
        title: "Short Title".to_string(),
        authors: vec!["Author 1".to_string()],
    };

    let book2 = Book {
        id: 2,
        title: "A Much Longer Title That Takes More Space".to_string(),
        authors: vec![
            "Author 1".to_string(),
            "Author 2".to_string(),
            "Author 3".to_string(),
        ],
    };

    assert_eq!(std::mem::size_of_val(&book1), std::mem::size_of_val(&book2));
}
