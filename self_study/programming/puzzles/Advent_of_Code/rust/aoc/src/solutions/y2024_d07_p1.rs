use std::fs;

const LINE_FEED: u8 = 10;
const DOT: u8 = 46;
const HASHTAG: u8 = 35;

const INPUT: &str = "y2024_d07.in";

pub fn main() {
    let input: String = fs::read_to_string(INPUT).unwrap();
    let mut res: u64 = 0;

    for line in input.lines() {
        let mut split = line.split(": ");
        let target: u64 = split.next().unwrap().parse::<u64>().unwrap();

        let mut numbers = split.next().unwrap().split_whitespace();
        let mut parsed: Vec<u64> = Vec::new();

        while let Some(v) = numbers.next() {
            let n = v.parse::<u64>().unwrap();
            parsed.push(n);
        }
        
        if is_equal_target(target, &parsed, 0, parsed[0]){
            res += target;
        }
    }

    assert_eq!(res, 2501605301465);
    print!("{res}");
}

fn is_equal_target(target: u64, numbers: &Vec<u64>, i: usize, res: u64) -> bool {
    while i + 1 < numbers.len() && res <= target {
        let i = i + 1;
        if is_equal_target(target, &numbers, i, res+numbers[i]) {
            return true;
        }
        if is_equal_target(target, &numbers, i, res*numbers[i]) {
            return true;
        }
        return false;
    }

    res == target
}
