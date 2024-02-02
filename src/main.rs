#[derive(Debug)]
struct Person {
    name: String,
    age: Option<u32>,
    email: Option<String>,
    address: Option<String>,
}

impl Person {
    fn new(name: String) -> PersonBuilder {
        PersonBuilder {
            name,
            age: None,
            email: None,
            address: None,
        }
    }   
}

// Define a PersonBuilder struct to implement the builder pattern
pub struct PersonBuilder {
    name: String,
    age: Option<u32>,
    email: Option<String>,
    address: Option<String>,
}

impl PersonBuilder {
    fn age(&mut self, age: u32) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn email(&mut self, email: &str) -> &mut Self {
        self.email = Some(String::from(email));
        self
    }

    fn address(&mut self, address: &str) -> &mut Self {
        self.address = Some(String::from(address));
        self
    }

    fn build(&mut self) -> Person {
        Person {
            name: self.name.clone(),
            age: self.age,
            email: self.email.clone(),
            address: self.address.clone(),
        }
    }
}

fn main() {
    // Creating a Person object using the builder pattern
    let person = Person::new("panos".to_string())
        .age(30)
        .email("john.doe@example.com")
        .address("123 Main St, City")
        .build();

    println!("{:?}", person);
}
