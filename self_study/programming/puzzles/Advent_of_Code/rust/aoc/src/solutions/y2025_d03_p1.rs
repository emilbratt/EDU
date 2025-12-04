const INPUT: &str = "y2025_d03.in";

pub fn main() {
    let bytes = std::fs::read(INPUT).unwrap();
    let input = parse_input(bytes);
    let ans = solve(&input);
    print!("{ans}");
}

fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>> {
    input.split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.iter()
                .map(|b| b-b'0')
                .collect()
        })
        .collect()
}

fn solve(input: &[Vec<u8>]) -> i64 {
    let mut total = 0;
    for row in input {
        total += line(&row);
    }
    total
}

fn line(row: &[u8]) -> i64 {
    let mut left: u8 = 0;
    let mut right: u8 = 0;
    let mut n: u8 = 9;
    while left * right == 0 && n > 0 {
        // Reset both to 0, then try next number (n). We do not need to check left number.
        (left, right) = (0, 0);

        for b in row {
            if left != 0 {
                if right > 0 {
                    if *b > right {
                        right = *b;
                    }
                } else {
                    right = *b;
                }
                continue;
            }

            if n as u8 == *b {
                if left == 0 {
                    left = *b;
                    continue;
                }
            }
        }

        n -= 1;
    }

    (left * 10 + right) as i64
}
