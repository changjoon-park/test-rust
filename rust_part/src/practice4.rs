use std::collections::HashMap;
use std::io::{self, Write};

struct Account {
    id: u32,
    balance: f64,
    is_active: bool,
}

impl Account {
    fn new(id: u32, balance: f64, is_active: bool) -> Self {
        Self {
            id,
            balance,
            is_active,
        }
    }

    fn withdraw(&mut self, amount: f64) -> Option<f64> {
        if self.is_active && self.balance >= amount {
            self.balance -= amount;
            Some(amount)
        } else {
            None
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut accounts = HashMap::new();
    accounts.insert(1, Account::new(1, 100.0, true));
    accounts.insert(2, Account::new(2, 50.0, false));
    accounts.insert(3, Account::new(3, 200.0, true));

    println!("Enter withdrawal requests (account_id amount), or 'done' to finish:");

    let mut successful_withdrawals = Vec::new();

    loop {
        let input = get_user_input("> ");
        if input.to_lowercase() == "done" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter account_id and amount.");
            continue;
        }

        let account_id: u32 = parts[0].parse().expect("Invalid account ID");
        let amount: f64 = parts[1].parse().expect("Invalid amount");

        let withdrawal = accounts
            .get_mut(&account_id)
            .and_then(|account| account.withdraw(amount));

        if let Some(amount) = withdrawal {
            successful_withdrawals.push((account_id, amount));
            println!("Withdrawal successful");
        } else {
            println!("Withdrawal failed");
        }
    }

    println!("\nSuccessful withdrawals:");
    for (id, amount) in successful_withdrawals {
        println!("Account {}: ${}", id, amount);
    }

    println!("\nFinal account balances:");
    for (id, account) in accounts.iter() {
        println!(
            "Account {}: ${} ({})",
            id,
            account.balance,
            if account.is_active {
                "active"
            } else {
                "inactive"
            }
        );
    }
}
