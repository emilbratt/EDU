use std::fs;

const INPUT: &str = "y2025_5.in";

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
        cur_col = (cur_col + 2) % line.chars().count();
    }
    assert_eq!(74, res);
    print!("{res}");
}
