use std::io;

fn main() {
    println!("Celsius to Fahrenheit Converter");

    loop {
        println!("Enter temperatures in Celsius separated by commans");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        input
            .split(',')
            .map(|temp| temp.trim().parse::<f64>())
            .for_each(|result| match result {
                Ok(celsius) => {
                    let fahrenheit = celsius_to_fahrenheit(celsius);
                    println!("{:.1}C is {:.1}F", celsius, fahrenheit);
                }
                Err(_) => println!("Error"),
            })
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
