use std::io;

fn main() {
    println!("Celsius to Fahrenheit Converter");

    loop {
        println!("Enter temperatures in Celsius separated by commas (or type 'exit' to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let temperatures: Vec<&str> = input.split(',').collect();
        let fahrenheit_temps: Vec<Result<f64, _>> = temperatures
            .iter()
            .map(|&temp| temp.trim().parse::<f64>())
            .collect();

        for result in fahrenheit_temps {
            match result {
                Ok(celsius) => {
                    let fahrenheit = celsius_to_fahrenheit(celsius);
                    println!("{:.1}°C is {:.1}°F", celsius, fahrenheit);
                }
                Err(_) => println!("Error: Invalid input"),
            }
        }
        input
            .split(',')
            .map(|temp| temp.trim().parse::<f64>())
            .for_each(|result| match result {
                Ok(celsius) => {
                    let fahrenheit = celsius_to_fahrenheit(celsius);
                    println!("{:.1}°C is {:.1}°F", celsius, fahrenheit);
                }
                Err(_) => println!("Error: Invalid input"),
            });
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
