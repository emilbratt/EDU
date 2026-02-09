pub fn main() {
    let mut input = std::fs::read_to_string("y2015_d13.in").unwrap();
    let happiness_data: Vec<(&str, i32, &str)> = parse_input(&input);

    let mut names: Vec<&str> = Vec::new();
    for (person, _, _) in happiness_data.iter() {
        if !names.contains(&person) {
            names.push(person);
        };
    }
    let unique_seatings = all_permutations(names);

    let mut ans = i32::MIN;
    for table in unique_seatings {
        let mut sum = 0;

        // So, the first and last element are also neighbours, so we do this first.
        let mut person_a = table[0];
        let mut person_b = table[table.len()-1];
        sum += get_happiness(person_a, person_b, &happiness_data);

        // And now, we take the adjacent persons in the array.
        for i in 1..table.len() {
            person_a = table[i-1];
            person_b = table[i];
            sum += get_happiness(person_a, person_b, &happiness_data);
        }
        ans = ans.max(sum);
    }

    print!("{ans}");
}

fn parse_input(input: &str) -> Vec<(&str, i32, &str)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
            let (a, gain, score, b) = (parts[0], parts[2], parts[3], parts[10].trim_end_matches("."));
            if gain == "gain" {
                (a, score.parse::<i32>().unwrap(), b)
            } else {
                (a, score.parse::<i32>().unwrap() * -1, b)
            }
        })
        .collect::<Vec<(&str, i32, &str)>>()
}

fn all_permutations<T: Clone>(mut arr: Vec<T>) -> Vec<Vec<T>> {
    let n = arr.len();
    let mut permutations = Vec::new();

    // Control array
    let mut c = vec![0usize; n];

    // First permutation
    permutations.push(arr.to_vec());

    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                arr.swap(0, i);
            } else {
                arr.swap(c[i], i);
            }

            permutations.push(arr.to_vec());

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    permutations
}

fn get_happiness(p1: &str, p2: &str, v: &[(&str, i32, &str)]) -> i32 {
    // Calculate the total happines between the two persons.
    let mut first_score: Option<i32> = None;
    for (person_a, score, person_b) in v {
        if p1 == *person_a && p2 == *person_b {
            match first_score {
                Some(first_score) => return score + first_score,
                None => first_score = Some(*score),
            }
        } else if p2 == *person_a && p1 == *person_b {
            match first_score {
                Some(first_score) => return score + first_score,
                None => first_score = Some(*score),
            }
        }
    }

    unreachable!("Happiness not found for either {p1} and {p2} or both :(");
}
