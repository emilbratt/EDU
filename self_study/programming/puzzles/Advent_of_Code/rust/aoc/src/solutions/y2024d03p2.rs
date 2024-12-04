use std::fs;

pub fn main() {
    let input: Vec<u8> = fs::read("y2024d03.in").unwrap();

    let mut res: i64 = 0;

    let mut enabled = true;

    let mut i = 0;
    while i < input.len() {
        if i + 4 <= input.len() && &input[i..i+4] == "mul(".as_bytes() {
            i += 4;

            // Max 3 digits for each factor + the comma ','.
            let mut exp = String::with_capacity(7);

            while i < input.len() && exp.len() <= 7 {
                let c = input[i] as char;
                i += 1;

                if c == ')' && enabled {
                    res += calc(&exp);
                    break;
                }

                match c {
                    '0'..='9' | ',' => exp.push(c),
                    _ => break,
                };
            }
        } else if i + 4 <= input.len() && &input[i..i+4] == "do()".as_bytes() {
            enabled = true;
            i += 4;
        } else if i + 7 <= input.len() && &input[i..i+7] == "don't()".as_bytes() {
            enabled = false;
            i += 7;
        } else {
            i += 1;
        }
    }

    print!("{res}");
}

fn calc(s: &str) -> i64 {
    assert!(s.contains(','));

    let mut _split = s.split(',');

    let l = _split.next().unwrap();
    let r = _split.next().unwrap();

    let f1 = l.parse::<i64>().unwrap();
    let f2 = r.parse::<i64>().unwrap();

    f1 * f2
}
