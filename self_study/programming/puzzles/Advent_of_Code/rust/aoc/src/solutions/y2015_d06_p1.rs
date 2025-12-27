pub const INPUT: &str = "y2015_d06.in";

enum Action {
    On,
    Off,
    Toggle,
}

struct Instruction {
    action: Action,
    row_from: usize,
    row_to: usize,
    col_from: usize,
    col_to: usize,
}

pub fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut lights: Vec<[bool; 1000]> = vec![[false; 1000]; 1000];
    let instructions = parse_input(&input);
    for ins in instructions {
        for row in ins.row_from..=ins.row_to {
            for col in ins.col_from..=ins.col_to {
                lights[row][col] = match ins.action {
                    Action::On => true,
                    Action::Off => false,
                    Action::Toggle => !lights[row][col],
                };
            }
        }
    }

    let ans: u32 = lights
        .into_iter()
        .map(|row| {
            // cast bools into u32 (0 or 1) and add them together..
            row.into_iter().fold(0, |acc: u32, b: bool| {
                acc + b as u32
            })
        }).collect::<Vec<u32>>()
        .iter()
        .sum();

    print!("{ans}");
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|mut l| {
            let mut split = l.rsplit(' ');
            let point_b = split.next().unwrap();
            let mut split_point = point_b.split(',');
            let (row_to, col_to) = (
                split_point.next().unwrap().parse::<usize>().unwrap(),
                split_point.next().unwrap().parse::<usize>().unwrap(),
            );
            let _ = split.next().unwrap();
            let point_a = split.next().unwrap();
            split_point = point_a.split(',');
            let (row_from, col_from) = (
                split_point.next().unwrap().parse::<usize>().unwrap(),
                split_point.next().unwrap().parse::<usize>().unwrap(),
            );
            let action = match split.next().unwrap() {
                    "on" => Action::On,
                    "off" => Action::Off,
                    "toggle" => Action::Toggle,
                    _ => unreachable!(),
            };
            Instruction { action, row_from, row_to, col_from, col_to }
        }).collect::<Vec<Instruction>>()
}
