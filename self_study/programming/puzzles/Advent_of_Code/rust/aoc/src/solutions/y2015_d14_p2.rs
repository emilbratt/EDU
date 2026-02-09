const TIME: usize = 2503;

pub fn main() {
    let input = std::fs::read_to_string("y2015_d14.in").unwrap();
    let reindeers = parse_input(&input);

    // [dist flown, flight seconds, resting seconds, points]
    let mut states: Vec<[usize; 4]> = vec![[0usize; 4]; reindeers.len()];
    let mut lead = 0;
    for _ in 0..TIME {
        for (state,reindeer) in states.iter_mut().zip(reindeers.iter()) {
            if state[1] < reindeer[1] {
                // in flight
                state[1] += 1; // add flight second
                state[0] += reindeer[0];
                lead = lead.max(state[0]);
            } else {
                // resting
                state[2] += 1; // add rest second
                if state[2] == reindeer[2] {
                    // resting finished, lets reset.
                    state[1] = 0;
                    state[2] = 0;
                }
            }
        }

        for state in states.iter_mut() {
            let dist = state[0];
            if dist == lead {
                state[3] += 1;
            };
        }

    }

    let ans = states.iter().map(|state| state[3]).max().unwrap();
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
