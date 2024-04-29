
use serde_json;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn run() {
    // Open and read the JSON file
    let path = "./src/file_handling/read_json_file/people.json";

    let mut file = File::open(path).unwrap();
    let mut json_str = String::new();

    file.read_to_string(&mut json_str).unwrap();

    // Parse the JSON string into a vector of Person structs
    let data: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    let people: &Vec<serde_json::Value> = data["people"].as_array().expect("Invalid JSON format");

    // Deserialize each person JSON object into a Person struct
    let mut persons: Vec<Person> = Vec::new();
    for person_json in people {
        let person: Person = serde_json::from_value(person_json.clone()).expect("Failed to deserialize JSON");
        persons.push(person);
    }

    println!("Parsed persons: {:?}", persons[0]);
    println!("Parsed persons: {:?}", persons[1]);
    println!("{} {}", persons[2].name, persons[2].age);

    let new_person = Person {
        name: persons[2].name.clone(),
        age: persons[2].age.clone(),
    };

    println!("Parsed persons: {:?}", new_person);
}
