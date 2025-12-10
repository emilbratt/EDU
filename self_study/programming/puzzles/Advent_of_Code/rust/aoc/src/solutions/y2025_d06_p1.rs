use core::num;

const INPUT: &str = "y2025_d06.in";

pub fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (numbers, operators) = parse_input(&input);
    let mut ans: i64 = 0;
    for i in 0..numbers[0].len() {
        let mut n: Vec<i64> = Vec::new();
        for j in 0..numbers.len() {
            n.push(numbers[j][i]);
        }
        ans += match operators[i] {
            '+' => n.iter().sum::<i64>(),
            '*' => n.iter().product::<i64>(),
            _ => unreachable!(),
        };
    }

    print!("{ans}");
}

fn parse_input(s: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut split = s.lines();
    while let Some(line) = split.next() {
        let mut sp = line.split_ascii_whitespace();
        let mut num: Vec<i64> = Vec::new();
        let mut is_operator = false;
        while let Some(part) = sp.next() {
            let c = part.chars().next().unwrap();
            if c == '+' || c == '*' {
                operators.push(c);
                is_operator = true;
            } else {
                num.push(
                    part.parse::<i64>().unwrap()
                )
            }
        }
        if !is_operator {
            numbers.push(num);
        }
    }
    
    (numbers, operators)
}
