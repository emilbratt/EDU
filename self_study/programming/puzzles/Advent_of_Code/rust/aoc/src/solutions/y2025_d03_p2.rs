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
    for bank in input {
        total += line(&bank, 12);
    }
    total
}

fn line(row: &[u8], n: usize) -> i64 {
    let mut batteries: Vec<i64> = vec![0; n];
    let mut lo = 0;
    let mut hi = row.len() - n;
    for batt in 0..n {
        for i in lo..=hi {
            let b = row[i] as i64;
            if batteries[batt] < b {
                batteries[batt] = b;
                lo = i + 1;
            }
        }
        hi += 1;
    }

    // Convert vec of bytes into i64, essentially doing [1, 2, 3] => 123.
    batteries.iter().fold(0, |acc, x| acc * 10 + x)
}
