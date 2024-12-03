use std::fs;

pub fn main() {
    let input = fs::read_to_string("y2024d02.in").unwrap();

    let mut res: i64 = 0;

    for line in input.lines() {
        let levels: Vec<i64> = line.split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

         // No need to check the full set, we only check N-1.
        let mut l: Vec<i64> = Vec::with_capacity(levels.len() - 1);

        for i in 0..levels.len() {
            for j in 0..levels.len() {
                // This is where we skip one select number for each iteration.
                if i == j { continue }

                l.push(levels[j]);
            }

            let mut n1 = l.pop().unwrap();
            let mut n2 = l.pop().unwrap();

            let incr: bool = n1 < n2;

            let mut safe = check_levels(incr, n1, n2);
            while safe {
                match l.pop() {
                    None => break,
                    Some(n) => {
                        n1 = n2;
                        n2 = n;
                        safe = check_levels(incr, n1, n2);
                    }
                }
            }
            if safe {
                res += 1;
                break;
            }

            l.clear();
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
