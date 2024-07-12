struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    fn celebrate_birthday(self) -> Self {
        Person {
            name: self.name,
            age: self.age + 1,
        }
    }

    fn double_age(self) -> Self {
        Person {
            name: self.name,
            age: self.age * 2,
        }
    }
}

fn parse_age(age_str: &str) -> Result<u32, String> {
    age_str.parse().map_err(|_| "Invalid age".to_string())
}

fn main() {
    // Case 1: map with method name
    let person_result = parse_age("30")
        .map(|age| Person::new("Alice".to_string(), age))
        .map(Person::double_age);

    match person_result {
        Ok(person) => println!(
            "Person with doubled age: {} is {} years old",
            person.name, person.age
        ),
        Err(e) => println!("Error: {}", e),
    }

    // Case 2: map with closure
    let person_result = parse_age("25")
        .map(|age| Person::new("Bob".to_string(), age))
        .map(|p| p.celebrate_birthday());

    match person_result {
        Ok(person) => println!(
            "Person after birthday: {} is {} years old",
            person.name, person.age
        ),
        Err(e) => println!("Error: {}", e),
    }
}
