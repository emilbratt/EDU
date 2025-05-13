use std::fs;

const INPUT: &str = "y2025_1.in";

pub fn main() {
    let input_str: String = fs::read_to_string(INPUT).unwrap();

    let mut res: usize = 0;
    for line in input_str.lines() {
        res += match (fits_sms(line), fits_tweet(line)) {
            (true, true) => 13,
            (true, false) => 11,
            (false, true) => 7,
            _ => 0,
        };
    }

    assert_eq!(107989, res);
    print!("{res}");
}

fn fits_sms(s: &str) -> bool {
    s.len() <= 160
}

fn fits_tweet(s: &str) -> bool {
    s.chars().count() <= 140
}
