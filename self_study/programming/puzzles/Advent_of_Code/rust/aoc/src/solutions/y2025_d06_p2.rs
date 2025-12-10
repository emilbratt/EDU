use core::num;
use std::hint::unreachable_unchecked;

const INPUT: &str = "y2025_d06.in";

pub fn main() {
    let input = std::fs::read(INPUT).unwrap();
    let grid = gridify(input);

    let mut ans: i64 = 0;
    let mut op: Option<char> = None;
    let mut numbers: Vec<i64> = Vec::new();
    let mut number = String::new();

    // Iterate from last column towards left.
    for i in (0..grid[0].len()).rev() {
        if let Some(_) = op {
            // This resets op to none, but more importantly it will
            // skip the empty whitespace that divides two blocks of numbers.
            op = None;
            continue;
        }

        // Iterate from first row and downwards,
        for j in 0..grid.len() {
            let c = grid[j][i];
            if j == grid.len()-1 && (c == '*' || c == '+') {
                op = Some(c);
            } else if c != ' ' {
                number.push(c);
            }
        }

        numbers.push(number.parse::<i64>().unwrap());
        number.clear();

        if let Some(op) = op {
            if numbers[0] == 0 {
                numbers.remove(0);
            }
            ans += match op {
                '+' => numbers.iter().sum::<i64>(),
                '*' => numbers.iter().product::<i64>(),
                _ => unreachable!(),
            };
            numbers.clear();
        }

    }
    print!("{ans}");
}

fn gridify(input: Vec<u8>) -> Vec<Vec<char>> {
    input.split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.iter().map(|c| *c as char)
                .collect()
        })
        .collect()
}