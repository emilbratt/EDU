use serde_json;
use serde::Deserialize;
use std::fs;

// make a struct that represents the json structure
#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

pub fn run() {
    let path = "./src/file_handling/read_json_file/person.json";

    let json_str = fs::read_to_string(path).unwrap();

    // Deserialize the JSON string into a Person struct
    let mut json_person: Person = serde_json::from_str(&json_str).expect("Failed to deserialize JSON");

    // Make a change, because we can.. :)
    json_person.set_age(60);

    println!("{:?}", json_person);
}
