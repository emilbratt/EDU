
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

    let data: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    let adults: &Vec<serde_json::Value> = data["adults"].as_array().expect("Invalid JSON format");

    // Deserialize each person JSON object into a Person struct
    let mut people: Vec<Person> = Vec::new();
    for person_json in adults {
        let person: Person = serde_json::from_value(person_json.clone()).expect("Failed to deserialize JSON");
        people.push(person);
    }

    println!("{:?}", people[0]);
    println!("{:?}", people[1]);
    println!("{} {}", people[2].name, people[2].age);

    let new_person = Person {
        name: people[2].name.clone(),
        age: people[2].age.clone(),
    };

    println!("{:?}", new_person);

    let _children: &Vec<serde_json::Value> = data["children"].as_array().expect("Invalid JSON format");
}
