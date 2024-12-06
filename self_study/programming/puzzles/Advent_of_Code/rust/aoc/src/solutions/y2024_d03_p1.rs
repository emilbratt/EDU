use std::fs;

pub fn main() {
    let input = fs::read_to_string("y2024_d03.in").unwrap();

    let mut res: i64 = 0;

    let mut split = input.split("mul(");

    while let Some(s) = split.next() {
        let mut _split = s.split(')');

        let inner_s = _split.next().unwrap();

        if !inner_s.contains(',') { continue }

        if has_invalid_char(inner_s) { continue }

        let mut _split = inner_s.split(',');

        let l = _split.next().unwrap();
        let r = _split.next().unwrap();

        let f1 = l.parse::<i64>().unwrap();
        let f2 = r.parse::<i64>().unwrap();

        res += f1 * f2;
    }

    print!("{res}");
}

pub fn has_invalid_char(s: &str) -> bool {
    for c in s.chars() {
        match c {
            '0'..='9' | ',' => (),
            _ => return true,
        }
    }

    false
}
