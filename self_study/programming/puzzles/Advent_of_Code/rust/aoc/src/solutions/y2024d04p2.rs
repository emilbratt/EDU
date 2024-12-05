use std::fs;

const INPUT: &str = "y2024d04.in";

const MAS: &[u8; 3] = &[77, 65, 83];
const SAM: &[u8; 3] = &[83, 65, 77];
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
    cols +=1; // we need to also account for line feed '\n'.

    for i in 0..input.len() {
        if diagonal(i, input, cols) {
            res += 1;
        }
    }

    res
}

fn diagonal(i: usize, v: &[u8], cols: usize) -> bool {
    let word_south_east: &[u8; 3] = match (v.get(i), v.get(i+cols+1), v.get(i+cols*2+2)) {
        (Some(b1) ,Some(b2) ,Some(b3)) => &[*b1 ,*b2 ,*b3],
        _ => return false,
    };

    let word_north_east: &[u8; 3] = match (v.get(i+cols*2), v.get(i+cols+1), v.get(i+2)) {
        (Some(b1) ,Some(b2) ,Some(b3)) => &[*b1 ,*b2 ,*b3],
        _ => return false,
    };

    match (word_south_east, word_north_east) {
        (MAS, MAS) => true,
        (MAS, SAM) => true,
        (SAM, MAS) => true,
        (SAM, SAM) => true,
        _ => false,
    }
}
