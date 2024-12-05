use std::fs;

use std::collections::HashMap;

const INPUT: &str = "y2024d05.in";

pub fn main() {
    let input: String = fs::read_to_string(INPUT).unwrap();

    let mut res: i64 = 0;

    let mut split = input.split("\n\n");

    // page ordering rules
    let pge_ord_rls: &str = split.next().unwrap();

    // page numbers of each update
    let pge_nums: &str = split.next().unwrap();

    let pge_ord_rls: HashMap<u8, Vec<u8>> = get_rules(pge_ord_rls);

    let mut unordered_pages: Vec<Vec<u8>> = Vec::new();
    for line in pge_nums.lines() {
        let pages = get_pages(line);
        if !is_ordered(&pages, &pge_ord_rls) {
            unordered_pages.push(pages.clone());
        }
    }

    for pages in unordered_pages.iter_mut() {
        for i in 0..pages.len() - 1 {
            for j in i+1..pages.len() {
                let (page, next_page) = (pages[i], pages[j]);

                let mut ordered = true;
                if let Some(pages) = pge_ord_rls.get(&page) {
                    if !pages.contains(&next_page) {
                        ordered = false;
                    }
                } else {
                    ordered = false;
                }

                if !ordered {
                    (pages[i], pages[j]) = (next_page, page);
                }
            }
        }

        res += pages[pages.len()/2] as i64;
    }

    print!("{res}");
}

fn is_ordered(pages: &Vec<u8>, pge_ord_rls: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 1..pages.len() {
        let (page, next_page) = (pages[i-1], pages[i]);

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

    for line in s.lines() {
        let mut split = line.split('|');

        let key = split.next().unwrap().parse::<u8>().unwrap();
        let value = split.next().unwrap().parse::<u8>().unwrap();

        rules.entry(key)
            .and_modify(|key| key.push(value))
            .or_insert(
                Vec::from([value])
            );
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
