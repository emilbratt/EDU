use std::fs;

const INPUT: &str = "y2024d04.in";

pub fn main() {
    let input: Vec<u8> = fs::read(INPUT).unwrap();
    let input: String = fs::read_to_string(INPUT).unwrap();

    print!("N/A");
}
