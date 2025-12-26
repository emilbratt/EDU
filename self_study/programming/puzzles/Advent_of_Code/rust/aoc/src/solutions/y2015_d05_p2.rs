use std::fs;

pub const INPUT: &str = "y2015_d05.in";

pub fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let mut ans = 0;
    'outer: for line in input.lines() {
        let bytes = line.as_bytes();

        // contains at least one letter which repeats with exactly one
        // letter between them
        for i in 0..bytes.len() {
            if i == bytes.len() - 2 {
                continue 'outer;
            }
            if bytes[i] == bytes[i+2] {
                break;
            }
        }

        // contains a pair of any two letters that appears at least
        // twice in the string without overlapping
        for i in 0..bytes.len()-1 {
            for j in i+2..bytes.len()-1 {
                if bytes[i..i+2] == bytes[j..j+2] {
                    ans += 1;
                    continue 'outer;
                }
            }
        }
    }

    print!("{ans}");
}
