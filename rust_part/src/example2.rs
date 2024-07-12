fn parse_and_multiply(num_str: &str) -> Result<i32, String> {
    let num = num_str.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(num * 2)
}

fn main() {
    match parse_and_multiply("5") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
