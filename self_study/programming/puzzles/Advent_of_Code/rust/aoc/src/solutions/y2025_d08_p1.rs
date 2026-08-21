const INPUT: &str = "y2025_d08.in";

pub fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let _boxes = parse_input(input);

    print!("d8 p1");
}

fn parse_input(input: String) -> Vec<(u32, u32, u32)> {
    input.lines().map(|l| {
        let mut split = l.split(',');
        (
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        )
    }).collect()
}
