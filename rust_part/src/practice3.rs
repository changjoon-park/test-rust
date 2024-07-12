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

    fn change_name(self, new_name: String) -> Self {
        Person {
            name: new_name,
            age: self.age,
        }
    }
}

fn parse_age(age_str: &str) -> Result<u32, String> {
    age_str.parse().map_err(|_| "Invalid age".to_string())
}

fn main() {
    // Case 1: map with function name (when possible)
    let age_result = parse_age("30");
    let person_result = age_result.map(|age| Person::new("Alice".to_string(), age));

    match person_result {
        Ok(person) => println!(
            "Person created: {} is {} years old",
            person.name, person.age
        ),
        Err(e) => println!("Error: {}", e),
    }

    // Case 2: map with closures
    let person_result = parse_age("25")
        .map(|age| Person::new("Bob".to_string(), age))
        .map(|p| p.celebrate_birthday())
        .map(|p| p.change_name("Robert".to_string()));

    match person_result {
        Ok(person) => println!(
            "Person modified: {} is {} years old",
            person.name, person.age
        ),
        Err(e) => println!("Error: {}", e),
    }
}
