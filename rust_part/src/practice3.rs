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

// New utility function that takes PersonType parameters
fn describe_relationship(parent: &PersonType, child: &PersonType) -> String {
    match (parent, child) {
        (PersonType::Adult(p), PersonType::Child(c)) => {
            format!(
                "{} (age {}) is the parent of {} (age {})",
                p.name, p.age, c.name, c.age
            )
        }
        (PersonType::Child(_), PersonType::Child(c)) => {
            format!(
                "Error: {} (age {}) cannot be a parent as they are a child",
                c.name, c.age
            )
        }
        (PersonType::Adult(p), PersonType::Adult(c)) => {
            format!(
                "{} (age {}) and {} (age {}) are both adults",
                p.name, p.age, c.name, c.age
            )
        }
        (PersonType::Child(p), PersonType::Adult(c)) => {
            format!(
                "Error: {} (age {}) cannot be the parent of {} (age {})",
                p.name, p.age, c.name, c.age
            )
        }
    }
}

fn main() {
    let name = "Alice";
    let age_str = "25";
    let parent = parse_age(age_str).map(|age| Person::new(name.to_string(), age));

    match &parent {
        Ok(person_type) => match person_type {
            PersonType::Adult(person) => {
                println!("{} is an adult, {} years old", person.name, person.age)
            }
            PersonType::Child(person) => {
                println!("{} is a child, {} years old", person.name, person.age)
            }
        },
        Err(e) => println!("Err: {}", e),
    }

    let child = parent.and_then(|person_type| match person_type {
        PersonType::Adult(person) => Ok(person.give_birth("Bob".to_string())),
        PersonType::Child(_) => Err("Children cannot give birth".to_string()),
    });

    match &child {
        Ok(person_type) => match person_type {
            PersonType::Adult(person) => {
                println!("{} is an adult, {} years old", person.name, person.age)
            }
            PersonType::Child(person) => {
                println!("{} is a child, {} years old", person.name, person.age)
            }
        },
        Err(e) => println!("Err: {}", e),
    }

    // Using the new utility function
    if let (Ok(parent_type), Ok(child_type)) = (&parent, &child) {
        println!("{}", describe_relationship(parent_type, child_type));
    }
}
