use std::fs;

const INPUT: &str = "y2024_d04.in";

// ASCII helper: https://typst.app/tools/ascii-table#q=XMAS
const XMAS: &[u8; 4] = &[88, 77, 65, 83];
const XMAS_REV: &[u8; 4] = &[83, 65, 77, 88];

const LF: &u8 = &10;

pub fn main() {
    let input: Vec<u8> = fs::read(INPUT).unwrap();
    let res = run(&input);

    print!("{res}");
}

fn run(input: &[u8]) -> i64 {
    let mut res: i64 = 0;

    let mut bytes = input.iter();

    let mut cols: usize = 0;
    while bytes.next().unwrap() != LF {
        cols += 1;
    }
    cols += 1; // we need to also account for line feed '\n'.

    for i in 0..input.len() {
        if horizontal(i, input, cols) {
            res += 1;
        }

        if vertical(i, input, cols) {
            res += 1;
        }

        res += diagonal(i, input, cols);
    }

    res
}

fn horizontal(i: usize, v: &[u8], cols: usize) -> bool {
    let word: &[u8; 4] = match (v.get(i), v.get(i+1), v.get(i+2), v.get(i+3)) {
        (Some(b1) ,Some(b2) ,Some(b3) ,Some(b4)) if b4 != LF => &[*b1 ,*b2 ,*b3 ,*b4],
        _ => return false,
    };

    match word {
        XMAS => true,
        XMAS_REV => true,
        _ => false,
    }
}

fn vertical(i: usize, v: &[u8], cols: usize) -> bool {
    let word: &[u8; 4] = match (v.get(i), v.get(i+cols), v.get(i+cols*2), v.get(i+cols*3)) {
        (Some(b1) ,Some(b2) ,Some(b3) ,Some(b4)) => &[*b1 ,*b2 ,*b3 ,*b4],
        _ => return false,
    };

    match word {
        XMAS => true,
        XMAS_REV => true,
        _ => false,
    }
}

fn diagonal(i: usize, v: &[u8], cols: usize) -> i64 {
    let word_south_west: &[u8; 4] = match (v.get(i), v.get(i+cols-1), v.get(i+cols*2-2), v.get(i+cols*3-3)) {
        (Some(b1) ,Some(b2) ,Some(b3) ,Some(b4)) => &[*b1 ,*b2 ,*b3 ,*b4],
        _ => return 0,
    };

    let word_south_east: &[u8; 4] = match (v.get(i), v.get(i+cols+1), v.get(i+cols*2+2), v.get(i+cols*3+3)) {
        (Some(b1) ,Some(b2) ,Some(b3) ,Some(b4)) => &[*b1 ,*b2 ,*b3 ,*b4],
        _ => return 0,
    };

    let mut count: i64 = 0;

    if word_south_west == XMAS {
        count += 1;
    }
    if word_south_west == XMAS_REV {
        count += 1;
    }
    if word_south_east == XMAS {
        count += 1;
    }
    if word_south_east == XMAS_REV {
        count += 1;
    }

    count
}
