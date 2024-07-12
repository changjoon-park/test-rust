fn parse_optional_int(input: &str) -> Option<i32> {
    input.trim().parse().ok()
}

fn parse_result_int(input: &str) -> Result<i32, String> {
    input
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("Failed to parse '{}'", input))
}

fn main() {
    let inputs = vec!["42", "not a number", "27"];

    for input in inputs {
        let result = parse_optional_int(input);
        let doubled = result.map(|num| num * 2);
        match doubled {
            Some(val) => println!("Parsed and doubled: {}", val),
            None => println!("Failed to parse '{}'", input),
        }
    }

    for input in inputs {
        let result = parse_result_int(input);
        let doubled = result.map(|num| num * 2);
        match doubled {
            Ok(val) => println!("Parsed and doubled: {}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}
