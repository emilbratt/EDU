use std::fs;

use std::collections::HashMap;

const INPUT: &str = "y2024_d05.in";

pub fn main() {
    let input: String = fs::read_to_string(INPUT).unwrap();

    let mut res: i64 = 0;

    let mut split = input.split("\n\n");

    // page ordering rules
    let pge_ord_rls = split.next().unwrap();

    // page numbers of each update
    let pge_nums = split.next().unwrap();

    let pge_ord_rls: HashMap<u8, Vec<u8>> = get_rules(pge_ord_rls);

    for line in pge_nums.lines() {
        let page_numbers = get_pages(line);
        if is_ordered(&page_numbers, &pge_ord_rls) {
            res += page_numbers[page_numbers.len()/2] as i64;
        }
    }

    print!("{res}");
}

fn is_ordered(page_numbers: &Vec<u8>, pge_ord_rls: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 1..page_numbers.len() {
        let (page, next_page) = (page_numbers[i-1], page_numbers[i]);

        if let Some(pages) = pge_ord_rls.get(&page) {
            if !pages.contains(&next_page) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn get_rules(s: &str) -> HashMap<u8, Vec<u8>> {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    // ..I found that using Vec<u8> was somehow faster than using a HashSet?..

    for line in s.lines() {
        let mut split = line.split('|');

        let key = split.next().unwrap().parse::<u8>().unwrap();
        let value = split.next().unwrap().parse::<u8>().unwrap();

        rules.entry(key)
            .and_modify(|key| key.push(value))
            .or_insert(Vec::from([value]));
    }

    rules
}

fn get_pages(s: &str) -> Vec<u8> {
    let mut split = s.split(',');

    let mut update: Vec<u8> = Vec::new();

    while let Some(n) = split.next() {
        update.push(n.parse::<u8>().unwrap());
    }

    update
}
