use std::io::{self, Write};

use dialoguer::{theme::ColorfulTheme, Input};

struct Item {
    name: String,
    price: Option<f64>,
}

impl Item {
    fn new(name: String, price: Option<f64>) -> Self {
        Self { name, price }
    }

    fn display(&self) {
        print!("{}: ", self.name);
        io::stdout().flush().unwrap();

        match self.price {
            Some(price) => println!("{:.2}", price),
            None => println!("Price not set"),
        }
    }

    fn apply_discount(&mut self, discount: f64) {
        self.price = self
            .price
            .map_or(None, |price| Some(price * (1.0 - discount)));
    }
}

fn main() {
    let mut inventory = Vec::new();

    loop {
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Item name: ")
            .interact_text()
            .unwrap();

        if name.to_lowercase() == "done" {
            break;
        }

        let price: f64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Price: ")
            .interact_text()
            .unwrap();

        inventory.push(Item::new(name.to_string(), Some(price)));
    }

    println!("Initial Inventory: ");
    for item in &inventory {
        item.display()
    }

    println!("20% discount !");
    for item in &mut inventory {
        item.apply_discount(0.2)
    }

    println!("After discount: ");
    for item in &inventory {
        item.display()
    }
}
