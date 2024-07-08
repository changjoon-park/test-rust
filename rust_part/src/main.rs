use std::io;

fn main() {
    println!("Celsius to Fahrenheit");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to Input");

        let input = input.trim();

        match input.parse::<f64>() {
            Ok(celsius) => {
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.1} C to {:.1} F", celsius, fahrenheit);
            }
            Err(_) => println!("Wrong number"),
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
