// Define a struct representing a Person
pub struct Person {
    name: String,
    age: u32,
}

pub trait DisplayInfo {
    fn display_info(&self);
}

// Implement methods associated with the Person struct
impl Person {
    // Constructor method to create a new Person instance
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    // Getter method to access the name field
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Getter method to access the age field
    pub fn get_age(&self) -> u32 {
        self.age
    }

    // Setter method to update the age field
    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

// Implement the DisplayInfo trait for the Person struct
impl DisplayInfo for Person {
    fn display_info(&self) {
        println!("Name: {}", self.get_name());
        println!("Age: {}", self.get_age());
    }
}
