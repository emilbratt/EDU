use std::fs;

const INPUT: &str = "y2025_3.in";

pub fn main() {
    let input_str: String = fs::read_to_string(INPUT).unwrap();

    let mut res: usize = 0;
    for line in input_str.lines() {
        if !valid_length(line) { continue }
        if !has_lower_case(line) { continue }
        if !has_upper_case(line) { continue }
        if !has_non_ascii(line) { continue }
        if !has_digit(line) { continue }

        res += 1;
    }
    print!("{res}");
}

fn valid_length(s: &str) -> bool {
    let count = s.chars().count();
    count >= 4 && count <= 12
}

fn has_lower_case(s: &str) -> bool {
    for c in s.chars() {
        if c.is_lowercase() {
            return true;
        }
    }
    false
}

fn has_upper_case(s: &str) -> bool {
    for c in s.chars() {
        if c.is_uppercase() {
            return true;
        }
    }
    false
}

fn has_non_ascii(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii() {
            return true;
        }
    }
    false
}

fn has_digit(s: &str) -> bool {
    for c in s.chars() {
        if c.is_digit(10) {
            return true;
        }
    }
    false
}
