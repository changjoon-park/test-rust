use std::collections::HashMap;
use std::io::{self, Write};

struct Account {
    id: u32,
    balance: f64,
    is_active: bool,
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

fn withdraw(account: &mut Account, amount: f64) -> Option<f64> {
    if account.is_active && account.balance >= amount {
        account.balance -= amount;
        Some(amount)
    } else {
        None
    }
}

fn main() {
    let mut accounts = HashMap::new();
    accounts.insert(
        1,
        Account {
            id: 1,
            balance: 100.0,
            is_active: true,
        },
    );
    accounts.insert(
        2,
        Account {
            id: 2,
            balance: 50.0,
            is_active: false,
        },
    );
    accounts.insert(
        3,
        Account {
            id: 3,
            balance: 200.0,
            is_active: true,
        },
    );

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
            .and_then(|account| withdraw(account, amount));

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
