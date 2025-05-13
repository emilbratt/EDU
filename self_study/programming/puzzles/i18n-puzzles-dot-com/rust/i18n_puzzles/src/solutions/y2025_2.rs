use std::fs;
use std::collections::HashMap;

use chrono::DateTime;

const INPUT: &str = "y2025_2.in";

pub fn main() {
    let input_str: String = fs::read_to_string(INPUT).unwrap();

    // Store key as unix epoch 'i64' instead of 'String' for faster access.
    let mut timestamps: HashMap<i64, u8> = HashMap::new();

    for l in input_str.lines() {
        match DateTime::parse_from_rfc3339(l) {
            Ok(t) => {
                let entry = timestamps.entry(t.timestamp()).or_insert(0);
                *entry += 1;
            }
            Err(e) => panic!("{}", e),
        }
    }

    for (key, val) in timestamps.into_iter() {
        if val >= 4 {
            let res = DateTime::from_timestamp(key, 0).unwrap().format("%Y-%m-%dT%H:%M:%S+00:00");
            assert_eq!("2020-10-25T01:30:00+00:00", res.to_string());
            print!("'{}'", res);
            return;
        }
    }

    panic!("gravitational wave was never recorded..")
}
