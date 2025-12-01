const INPUT: &str = "y2025_d01.in";

pub fn main() {
    let input: Vec<u8> = std::fs::read(INPUT).unwrap();
    print!("{}", solve(&input));
}

fn solve(input: &[u8]) -> i32 {
    let mut res = 0;

    let mut position: i32 = 50; // Dial position.
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
            let mut v = s.parse::<i32>().unwrap();

            // This solution is obviously very slow and can be much faster.
            // I might write a better/faster solution later. :)
            while v > 0 {
                position += if left { -1 } else { 1 };

<<<<<<< HEAD
                if position < 0 { position = 99; }
                else if position > 99 { position = 0 }
=======
                if position < 0 {
                    position = 99;
                } else if position > 99 {
                    position = 0;
                }
>>>>>>> 9059ef4 (AoC Day 1 Part 2 solved)

                if position == 0 { res += 1 }
                v -= 1;
            }

            s.clear();
        }
    }

    res
}
