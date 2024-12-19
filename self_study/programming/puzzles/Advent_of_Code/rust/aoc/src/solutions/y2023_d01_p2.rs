const INPUT: &str = "y2023_d01.in";

const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

const NUMBERS_REV: [&str; 10] = ["orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

pub fn main() {
    let input: String = std::fs::read_to_string(INPUT).unwrap();

    let res: i64 = input.lines()
        .map(|line| handle_line(line))
        .fold(0, |acc, x| acc + x);


    print!("{res}");
}

fn handle_line(s: &str) -> i64 {
    let mut c_iter = s.chars();
    let mut string_acc = String::new();
    let mut first: Option<i64> = None;
    while first.is_none() {
        let c = c_iter.next().unwrap();

        if c.is_numeric() {
            first = Some(c.to_digit(10).unwrap() as i64);
        } else {
            string_acc.push(c);
            if string_acc.len() >= 3 {
                let mut i: usize = 0;
                while i < NUMBERS.len() && first.is_none() {
                    if string_acc.contains(NUMBERS[i]) {
                        first = Some(i as i64);
                    }
                    i += 1;
                }
            }
        }
    }

    let mut c_iter = s.chars().rev();
    string_acc.clear();
    let mut last: Option<i64> = None;
    while last.is_none() {
        let c = c_iter.next().unwrap();

        if c.is_numeric() {
            last = Some(c.to_digit(10).unwrap() as i64);
        } else {
            string_acc.push(c);
            if string_acc.len() >= 3 {
                let mut i: usize = 0;
                while i < NUMBERS_REV.len() && last.is_none() {
                    if string_acc.contains(NUMBERS_REV[i]) {
                        last = Some(i as i64);
                    }
                    i += 1;
                }
            }
        }
    }

    first.unwrap() * 10 + last.unwrap()
}