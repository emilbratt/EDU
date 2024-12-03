use std::fs;

pub fn main() {
    let input: Vec<u8> = fs::read("y2015d01.in").unwrap();

    let mut res: i64 = 0;

    let mut i = 0;
    while res >= 0 {
        let b = input[i];

        match b {
            40 => res += 1, // 40 is ascii for (
            41 => res -= 1, // 41 is ascii for )
            _ => panic!(),
        }

        i += 1;
    }

    print!("{i}");
}
