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
}

fn find_person(id: u32) -> Option<Person> {
    if id == 1 {
        Some(Person::new("Alice".to_string(), 30))
    } else {
        None
    }
}

fn main() {
    // Case 1: map with method name
    let doubled_age_person = find_person(1).map(Person::double_age);

    match doubled_age_person {
        Some(person) => println!(
            "Person with doubled age: {} is {} years old",
            person.name, person.age
        ),
        None => println!("Person not found"),
    }

    // Case 2: map with closure
    let older_person = find_person(1).map(|p| Person::new(p.name, p.age + 5));

    match older_person {
        Some(person) => println!("Older person: {} is {} years old", person.name, person.age),
        None => println!("Person not found"),
    }

    // Case 3: map on None
    let not_found = find_person(2).map(Person::double_age);

    match not_found {
        Some(_) => println!("This won't be printed"),
        None => println!("Person not found, map wasn't called"),
    }
}
