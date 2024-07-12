struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    fn double_age(self) -> Self {
        Person {
            name: self.name,
            age: self.age * 2,
        }
    }

    fn celebrate_birthday(self) -> Self {
        Person {
            name: self.name,
            age: self.age + 1,
        }
    }
}

fn parse_age(age_str: &str) -> Result<u32, String> {
    age_str.parse().map_err(|_| "Invalid age".to_string())
}

fn find_person(id: u32) -> Option<Person> {
    if id == 1 {
        Some(Person::new("Alice".to_string(), 30))
    } else {
        None
    }
}

fn main() {
    // Result::map() examples
    println!("- Result::map() examples:");

    // Using map with a method name
    let person_result = parse_age("30")
        .map(|age| Person::new("Bob".to_string(), age))
        .map(Person::double_age);

    match person_result {
        Ok(person) => println!(
            "Person with doubled age: {} is {} years old",
            person.name, person.age
        ),
        Err(e) => println!("Error: {}", e),
    }

    // Using map with a closure
    let person_result = parse_age("25")
        .map(|age| Person::new("Charlie".to_string(), age))
        .map(|p| p.celebrate_birthday());

    match person_result {
        Ok(person) => println!(
            "Person after birthday: {} is {} years old",
            person.name, person.age
        ),
        Err(e) => println!("Error: {}", e),
    }

    // Option::map() examples
    println!("\n- Option::map() examples:");

    // Using map with a method name
    let doubled_age_person = find_person(1).map(Person::double_age);

    match doubled_age_person {
        Some(person) => println!(
            "Person with doubled age: {} is {} years old",
            person.name, person.age
        ),
        None => println!("Person not found"),
    }

    // Using map with a closure
    let older_person = find_person(1).map(|p| Person::new(p.name, p.age + 5));

    match older_person {
        Some(person) => println!("Older person: {} is {} years old", person.name, person.age),
        None => println!("Person not found"),
    }

    // Demonstrating map on None
    let not_found = find_person(2).map(Person::double_age);

    match not_found {
        Some(_) => println!("This won't be printed"),
        None => println!("Person not found, map wasn't called"),
    }
}
