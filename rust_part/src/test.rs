use std::io;

fn main() {
    println!("Celsius to Fahrenheit Converter");

    loop {
        println!("Enter a temperature in Celsius (or 'q' to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.to_lowercase() == "q" {
            break;
        }

        match input.parse::<f64>() {
            Ok(celsius) => {
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.1}°C is {:.1}°F", celsius, fahrenheit);
            }
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
