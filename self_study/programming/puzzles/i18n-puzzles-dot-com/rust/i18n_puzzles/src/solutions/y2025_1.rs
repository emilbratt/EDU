use std::fs;

const INPUT: &str = "y2025_1.in";

pub fn main() {
    let input_str: String = fs::read_to_string(INPUT).unwrap();

    let mut res: usize = 0;
    for line in input_str.lines() {
        match (valid_sms(line), valid_tweet(line)) {
            (true, true) => res += 13,
            (true, false) => res += 11,
            (false, true) => res += 7,
            _ => (),
        }
    }
    print!("{res}");
}

fn valid_sms(s: &str) -> bool {
    s.len() <= 160
}

fn valid_tweet(s: &str) -> bool {
    s.chars().count() <= 140
}
