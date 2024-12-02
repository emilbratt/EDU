use std::fs;

pub fn main() {
    let input = fs::read_to_string("y2024d02.in").unwrap();

    let mut res: i64 = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let n1 = split.next().unwrap();
        // we expect at least two numbers each line
        let n2 = split.next().unwrap();

        let mut n1 = n1.parse::<i64>().unwrap();
        let mut n2 = n2.parse::<i64>().unwrap();

        let incr: bool = n1 < n2;

        let mut safe = check_levels(incr, n1, n2);
        while safe {
            match split.next() {
                None => break,
                Some(n) => {
                    n1 = n2;
                    n2 = n.parse::<i64>().unwrap();
                    safe = check_levels(incr, n1, n2);
                }
            }
        }

        if safe {
            res += 1;
        }
    }

    print!("{res}");
}

fn check_levels(incr: bool, n1: i64, n2: i64) -> bool {
    match n2-n1 {
        1..=3   => incr,
        -3..=-1 => !incr,
        _ => false,
    }
}
