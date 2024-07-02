use std::io;

fn main() {
    println!("Simple Calculator");

    loop {
        let mut input = String::new();

        println!("Enter an expression (e.g., 5 + 3) or 'q' to quit:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_lowercase() == "q" {
            break;
        }

        match calculate(&input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn calculate(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Invalid input format".to_string());
    }

    let a = parts[0]
        .parse::<f64>()
        .map_err(|_| "Invalid first number")?;
    let op = parts[1];
    let b = parts[2]
        .parse::<f64>()
        .map_err(|_| "Invalid second number")?;

    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}
