use std::fs;

pub const INPUT: &str = "y2015_d05.in";

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let mut ans = 0;
    'outer: for line in input.lines() {
        let mut n = 0;
        let mut twice = false;
        for s in FORBIDDEN {
            if line.contains(s) {
                continue 'outer;
            }
        }
        let bytes = line.as_bytes();
        for i in 0..bytes.len() {
            if i < bytes.len() - 1 {
                if bytes[i] == bytes[i+1] {
                    twice = true;
                }
            }
            if VOWELS.contains(&(bytes[i] as char)) {
                n += 1;
            }
            if n >= 3 && twice {
                ans += 1;
                continue 'outer;
            }
        }
    }


    print!("{ans}");
}
