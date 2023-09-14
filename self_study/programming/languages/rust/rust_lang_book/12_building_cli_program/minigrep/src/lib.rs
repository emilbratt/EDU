use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // The last ? will return the error value from the current function for the caller to handle.

    println!("With text:\n{contents}");

    let empty_value = (); // The () implies no value e.g. no particular value.
    Ok(empty_value) // Returns an Ok value (empty) in the success case.
}
