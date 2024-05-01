pub mod types;
pub mod person;
pub mod people;

/*
    Cargo.toml

    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
*/

pub fn run() {
    person::run();
    people::run();
    types::run();
}
