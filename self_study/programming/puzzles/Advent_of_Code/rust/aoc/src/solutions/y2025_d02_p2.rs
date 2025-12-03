const INPUT: &str = "y2025_d02.in";
use std::{thread, time};
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
        for s in *left..=*right {
            let b = s.to_string().chars().map(|c| c as u8 - 48).collect::<Vec<u8>>();
            if let Some(n) = check(&b) {
                found.push(n);
            }
        }
    }

    found.into_iter().sum::<u64>()
}

fn check(b: &[u8]) -> Option<u64> {
    let mut digits: Vec<i64> = Vec::new();
    for div in (1..=b.len()/2) {
        if b.len() % div == 0 {
            let mut invalid = true;
            let first_chunk = &b[0..div];
            for i in (div..b.len()).step_by(div) {
                let next_chunk = &b[i..i+div];
                if first_chunk != next_chunk {
                    invalid = false;
                }
            }
            if invalid {
                return Some(slice_u8_to_u64(b));
            }
        }
    }

    None
}

fn slice_u8_to_u64(bytes: &[u8]) -> u64 {
    let mut n = 0_u64;

    for b in bytes {
        n = n * 10 + *b as u64;
    }
    n
}
