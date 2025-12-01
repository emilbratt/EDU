const INPUT: &str = "y2025_d01.in";

pub fn main() {
    let input: Vec<u8> = std::fs::read(INPUT).unwrap();
    print!("{}", solve(&input));
}

fn solve(input: &[u8]) -> i32 {
    let mut res = 0;

    let mut position: i32 = 50; // Dial position.
    let wrap_around: i32 = 100; // Dial goes from 0 to 99 = 100 values so we do modulo 100 to simulate dialing.
    let mut s = String::new(); // Parsed value here..
    let mut left = true; // Simluates dialing left => we represent it as negative value

    for b in input.iter() {
        if *b as char != '\n' {
            match *b as char {
                'R' => left = false,
                'L' => left = true,
                v => s.push(*b as char),
            }
        } else {
            let v = s.parse::<i32>().unwrap();
            position += if left { -v } else { v };
            if position < 0 || position > 99 {
                position = position % wrap_around;
            }
            if position == 0 { res += 1 }
            s.clear();
        }
    }

    res
}
