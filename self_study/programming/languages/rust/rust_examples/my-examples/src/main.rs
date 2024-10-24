

pub mod date_handling;
pub mod error_handling;
pub mod file_handling;
pub mod generics_handling;


fn main() {
    date_handling::examples::run();

    error_handling::examples::run();

    file_handling::read_json_file::run();
    file_handling::read_text_file::run();

    generics_handling::structs::run();
}
