use std::collections::{HashMap, HashSet};

pub fn main() {
    let input = std::fs::read_to_string("y2015_d09.in").unwrap();

    let mut distances = parse_input(&input);
    let mut cities: HashSet<&str> = HashSet::new();
    for d in distances.iter() {
        cities.insert(d.0);
        cities.insert(d.1);
    }
    let cities = cities.into_iter().collect::<Vec<&str>>();
    let all_permutations = all_permutations(cities);

    let mut ans = u32::MAX;
    for arr in all_permutations {
        let mut count = 0;
        for i in 0..arr.len()-1 {
            let (from, to) = (arr[i], arr[i+1]);
            let mut found = false;
            for (city_a, city_b, n) in distances.iter() {
                if (from == *city_a && to == *city_b)
                || (from == *city_b && to == *city_a) {
                    count += n;
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("Distance '{from}' to '{to}' was not found");
            }
        }
        if ans > count {
            ans = count;
        }
    }

    print!("{ans}");
}

fn parse_input(input: &str) -> Vec<(&str, &str, u32)> {
    input.lines().map(|l| {
        let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
        let (from, to, dist) = (parts[0], parts[2], parts[4]);
        (from, to, dist.parse::<u32>().unwrap())
    }).collect::<Vec<(&str, &str, u32)>>()
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
