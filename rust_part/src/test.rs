use std::io;

struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(celsius: f64) -> Self {
        Self { celsius }
    }

    fn from_fahrenheit(fahrenheit: f64) -> Self {
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        Self { celsius }
    }

    fn to_fahrenheit(&self) -> f64 {
        (self.celsius * 9.0 / 5.0) + 32.0
    }

    fn to_string(&self) -> String {
        format!("{:.1}°C is {:.1}°F", self.celsius, self.to_fahrenheit())
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

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Split the input string into parts, each part is a &str
        input
            .split(',')
            .map(|temp| {
                let temp = temp.trim(); // temp is a &str
                if temp.ends_with('C') || temp.ends_with('c') {
                    let celsius = temp[..temp.len() - 1].trim().parse::<f64>();
                    celsius
                        .map(Temperature::new)
                        .map_err(|_| "Invalid format".to_string())
                } else if temp.ends_with('F') || temp.ends_with('f') {
                    let fahrenheit = temp[..temp.len() - 1].trim().parse::<f64>();
                    fahrenheit
                        .map(Temperature::from_fahrenheit)
                        .map_err(|_| "Invalid format".to_string())
                } else {
                    Err("Invalid format".to_string())
                }
            })
            .for_each(|result| match result {
                Ok(temp) => {
                    println!("{}", temp.to_string());
                }
                Err(err) => println!("Error: {}", err),
            });
    }
}
