use std::io::{self, Write};

// Define a struct to represent an item
struct Item {
    name: String,
    price: Option<f64>,
}

impl Item {
    // Function to create a new item
    fn new(name: String, price: Option<f64>) -> Self {
        Self { name, price }
    }

    // Function to display item details
    fn display(&self) {
        print!("{}: ", self.name);
        match self.price {
            Some(price) => println!("${:.2}", price),
            None => println!("Price not set"),
        }
    }

    // Function to apply a discount if the item has a price
    fn apply_discount(&mut self, discount: f64) {
        self.price = self.price.map_or(None, |p| Some(p * (1.0 - discount)));
    }
}

fn main() {
    let mut inventory = Vec::new();

    loop {
        println!("\nEnter item details (or press Enter to finish):");

        print!("Item name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if name.is_empty() {
            break;
        }

        print!("Item price (leave blank if not set): ");
        io::stdout().flush().unwrap();
        let mut price_str = String::new();
        io::stdin().read_line(&mut price_str).unwrap();
        let price_str = price_str.trim();

        let price = if price_str.is_empty() {
            None
        } else {
            match price_str.parse::<f64>() {
                Ok(p) => Some(p),
                Err(_) => {
                    println!("Invalid price. Setting price to None.");
                    None
                }
            }
        };

        inventory.push(Item::new(name.to_string(), price));
    }

    println!("\nInitial Inventory:");
    inventory.iter().for_each(|item| item.display());

    println!("\nApplying 20% discount:");
    inventory
        .iter_mut()
        .for_each(|item| item.apply_discount(0.2));

    println!("\nUpdated Inventory:");
    inventory.iter().for_each(|item| item.display());
}
