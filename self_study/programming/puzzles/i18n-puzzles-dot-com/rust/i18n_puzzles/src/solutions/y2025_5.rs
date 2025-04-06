use std::fs;

use chrono::{DateTime, NaiveDateTime};

const INPUT: &str = "y2025_5.in";
const STEPS_RIGHT: usize = 2;

pub fn main() {
    let input_str: String = fs::read_to_string(INPUT).unwrap();

    let mut res = 0;
    let mut cur_col = 0;
    for line in input_str.lines() {
        for (i, c) in line.chars().enumerate() {
            if i == cur_col && c == '💩' {
                res += 1;
            }
        }
        cur_col = (cur_col + STEPS_RIGHT) % line.chars().count();
    }
    print!("{res}");
}
