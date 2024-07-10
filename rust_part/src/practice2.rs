use std::collections::HashMap;
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

        let temperatures = input
            .split(',')
            .map(str::trim)
            .filter_map(|temp_str| match parse_temperature(temp_str) {
                Ok(temp) => Some(temp),
                Err(err) => {
                    println!("Error: {} for input '{}'", err, temp_str);
                    None
                }
            })
            .collect::<Vec<_>>();

        if temperatures.is_empty() {
            println!("No valid temperatures entered.");
            continue;
        }

        // Print all valid temperatures
        println!("Valid temperatures:");
        temperatures.iter().for_each(|temp| println!("{}", temp));

        // Calculate and print average Celsius temperature
        let avg_celsius: f32 =
            temperatures.iter().map(|t| t.celsius).sum::<f32>() / temperatures.len() as f32;
        println!("Average temperature: {:.1}°C", avg_celsius);

        // Find and print the highest and lowest temperatures
        if let (Some(min), Some(max)) = (
            temperatures
                .iter()
                .min_by(|a, b| a.celsius.partial_cmp(&b.celsius).unwrap()),
            temperatures
                .iter()
                .max_by(|a, b| a.celsius.partial_cmp(&b.celsius).unwrap()),
        ) {
            println!("Lowest temperature: {}", min);
            println!("Highest temperature: {}", max);
        }

        // Group temperatures by scale
        let grouped = temperatures.iter().fold(HashMap::new(), |mut acc, temp| {
            acc.entry(if temp.to_fahrenheit() == temp.celsius {
                "Celsius"
            } else {
                "Fahrenheit"
            })
            .or_insert_with(Vec::new)
            .push(temp);
            acc
        });

        // Print grouped temperatures
        for (scale, temps) in grouped.iter() {
            println!("{} temperatures:", scale);
            temps.iter().for_each(|t| println!("  {}", t));
        }
    }
}
