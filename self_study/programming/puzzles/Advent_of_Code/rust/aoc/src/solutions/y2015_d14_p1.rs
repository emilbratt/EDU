const TIME: usize = 2503;

pub fn main() {
    let input = std::fs::read_to_string("y2015_d14.in").unwrap();
    let reindeers = parse_input(&input);

    let mut ans = 0;
    for reindeer in reindeers.iter() {
        let mut time = 0;
        let mut km = 0;
        let (dist, flight_duration, rest_duration) = (reindeer[0], reindeer[1], reindeer[2]);
        while time + flight_duration <= TIME {
            km += dist * flight_duration;
            time += flight_duration;
            time += rest_duration;
        }
        ans = ans.max(km);
    }

    print!("{ans}");
}

fn parse_input(input: &str) -> Vec<[usize; 3]> {
    //km, duration, rest
    let mut reindeer = [0usize; 3];

    input.lines().map(|l| {
        let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
        reindeer[0] = parts[3].parse::<usize>().unwrap();
        reindeer[1] = parts[6].parse::<usize>().unwrap();
        reindeer[2] = parts[13].parse::<usize>().unwrap();
        reindeer
    }).collect::<Vec<[usize; 3]>>()
}
