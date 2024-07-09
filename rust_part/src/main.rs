use std::fmt;
use std::io;

struct Temperature {
    celsius: f32,
}

impl Temperature {
    fn new(celsius: f32) -> Self {
        Self { celsius }
    }

    fn from_fahrenheit(fahrenheit: f32) -> Self {
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        Self { celsius }
    }

    fn to_fahrenheit(&self) -> f32 {
        (self.celsius * 9.0 / 5.0) + 32.0
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C is {:.1}°F", self.celsius, self.to_fahrenheit())
    }
}

fn parse_temperature(input: &str) -> Result<Temperature, String> {
    let input = input.trim();
    match input.chars().last() {
        Some('C') | Some('c') => input[..input.len() - 1]
            .trim()
            .parse()
            .map(Temperature::new)
            .map_err(|_| "Invalid format for Celsius temperature".to_string()),
        Some('F') | Some('f') => input[..input.len() - 1]
            .trim()
            .parse()
            .map(Temperature::from_fahrenheit)
            .map_err(|_| "Invalid format for Fahrenheit temperature".to_string()),
        _ => Err("Temperature must end with 'C' or 'F'".to_string()),
    }
}

fn main() {
    println!("Temperature Converter (Celsius to Fahrenheit and vice versa)");
    loop {
        println!("Enter temperatures with suffix 'C' for Celsius or 'F' for Fahrenheit separated by commas (or type 'exit' to quit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        input
            .split(',')
            .map(str::trim)
            .for_each(|temp_str| match parse_temperature(temp_str) {
                Ok(temp) => println!("{}", temp),
                Err(err) => println!("Error: {}", err),
            });
    }
}
