const INPUT: &str = "y2025_d02.in";

pub fn main() {
    let bytes = std::fs::read(INPUT).unwrap();

    let ranges = parse_input(&bytes);

    print!("{}", solve(&ranges));
}

fn parse_input(bytes: &[u8]) -> Vec<(i64,i64)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    let mut i = 0;
    let total = bytes.len();

    while i < total {
        let mut left = 0i64;
        while i < total && bytes[i] != b'-' && bytes[i] != b'\n' && bytes[i] != b'\r' {
            if bytes[i] != b',' {
                left = left * 10 + (bytes[i] - b'0') as i64;
            }
            i += 1;
        }

        if i < total && bytes[i] == b'-' {
            i += 1;
        }

        let mut right = 0i64;
        while i < total && bytes[i] != b',' && bytes[i] != b'\n' && bytes[i] != b'\r' {
            right = right * 10 + (bytes[i] - b'0') as i64;
            i += 1;
        }

        ranges.push((left, right));

        // skip these characters: ',' '\n' '\r'
        while i < total && (bytes[i] == b',' || bytes[i] == b'\n' || bytes[i] == b'\r') {
            i += 1;
        }
    }

    ranges
}

fn solve(ranges: &Vec<(i64,i64)>) -> u64 {
    let mut found: Vec<u64> = Vec::new();

    for (left, right) in ranges {
        // Odd amount of digits will never produce a repeating sequence..
        if left.to_string().len() == right.to_string().len() {
            if left.to_string().len() % 2 != 0 {
                continue;
            }
        }

        for i in *left..=*right {
            let cc = i.to_string().chars().map(|c| c as u8 - 48).collect::<Vec<u8>>();

            // Odd amount of digits will never produce a repeating sequence..
            if cc.len() % 2 != 0 { continue; }

            let half = cc.len() / 2;

            // This checks if we have identical sequence..
            if cc[0..half] == cc[half..half+half] {
                let mut n = 0_u64;
                let ccc: &[u8] = &cc[0..half+half];
                for b in ccc {
                    n = n * 10 + *b as u64;
                }
                found.push(n);
            }
        }
    }

    found.into_iter().sum::<u64>()
}
