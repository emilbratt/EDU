use std::error::Error;
use std::fs;

pub fn print_file_content(path: &str) {
    let contents = fs::read_to_string(path).unwrap(); // panic on fail by calling .unwrap()

    println!("{}", contents);
}

pub fn get_content_as_result_with_questionmark_operator(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?; // ? => if it fails, stop here and return error

    Ok(contents)
}

pub fn get_content_as_result_with_match(path: &str) -> Result<String, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    Ok(content)
}

pub fn get_content_as_empty_string_if_failed(path: &str) -> String {
    let content = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => String::new(),
    };

    content
}
