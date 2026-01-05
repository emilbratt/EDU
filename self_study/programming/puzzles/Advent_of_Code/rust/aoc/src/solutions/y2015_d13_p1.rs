pub fn main() {
    let mut input = std::fs::read_to_string("y2015_d13.in").unwrap();
    let v = parse_input(&input);

    for l in v.iter() {
        println!("{}: {} with {}", l.0, l.1, l.2);
    }

    let mut ans = 0;

    print!("{ans}");
}

fn parse_input(input: &str) -> Vec<(&str, i32, &str)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
            let (a, gain, n, b) = (parts[0], parts[2], parts[3], parts[10]);
            if gain == "gain" {
                (a, n.parse::<i32>().unwrap(), b)
            } else {
                (a, n.parse::<i32>().unwrap() * -1, b)
            }
        })
        .collect::<Vec<(&str, i32, &str)>>()
}
