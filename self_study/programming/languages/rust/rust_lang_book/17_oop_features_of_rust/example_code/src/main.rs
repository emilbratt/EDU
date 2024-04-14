pub mod encapsulate;

use encapsulate::{Person, DisplayInfo};

fn main() {
    // Create a new Person instance
    let mut person = Person::new(String::from("Alice"), 30);

    // Display information about the person
    println!("Initial Info:");
    person.display_info();

    // Update the person's age
    person.set_age(35);

    // Display updated information about the person
    println!("\nUpdated Info:");
    person.display_info();

}
