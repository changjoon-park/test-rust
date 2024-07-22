enum PersonType {
    Adult(Person),
    Child(Person),
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> PersonType {
        if age >= 18 {
            PersonType::Adult(Self { name, age })
        } else {
            PersonType::Child(Self { name, age })
        }
    }

    fn give_birth(&self, name: String) -> PersonType {
        PersonType::Child(Self { name, age: 0 })
    }

    fn celebrate_birth(&mut self) {
        self.age += 1;
    }
}

fn parse_age(age_str: &str) -> Result<u32, String> {
    age_str
        .parse::<u32>()
        .map_err(|_| "Invalid Input".to_string())
}

fn main() {
    let name = "Alice";
    let age_str = "25";
    let parent = parse_age(age_str).map(|age| Person::new(name.to_string(), age));

    match &parent {
        Ok(PersonType::Adult(person)) => {
            println!("{} is an adult, {} years old", person.name, person.age);
        }
        Ok(PersonType::Child(person)) => {
            println!("{} is a child, {} years old", person.name, person.age);
        }
        Err(e) => println!("Err: {}", e),
    }

    let child = parent.and_then(|person_type| match person_type {
        PersonType::Adult(person) => Ok(person.give_birth("Bob".to_string())),
        PersonType::Child(_) => Err("Children cannot give birth".to_string()),
    });

    match &child {
        Ok(PersonType::Adult(person)) => {
            println!("{} is an adult, {} years old", person.name, person.age);
        }
        Ok(PersonType::Child(person)) => {
            println!("{} is a child, {} years old", person.name, person.age);
        }
        Err(e) => println!("Err: {}", e),
    }
}
