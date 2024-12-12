use std::fs;
use std::collections::HashMap;

const CHAR_NUMBER_OFFSET: u8 = 48;
const DOT: u8 = 46;
const HASHTAG: u8 = 35;
const LINE_FEED: u8 = 10;

const INPUT: &str = "y2024_d09.in";

const FILE_ID_OFFSET: usize = 1;
const IS_FREE_BLOCK: usize = 0;

pub fn main() {
    assert!(FILE_ID_OFFSET > 0);
    assert!(IS_FREE_BLOCK == 0);

    let mut input: Vec<u8> = fs::read(INPUT).unwrap();

    let mut blocks = get_disk_layout(input);

    // Move file blocks.
    let mut i = 0;
    let mut j = blocks.len() - 1;
    loop {
        while blocks[i] != IS_FREE_BLOCK {
            i += 1;
        }
        while blocks[j] == IS_FREE_BLOCK {
            j -= 1;
        }

        if i >= j {
            break;
        }

        blocks.swap(i, j);

        i += 1;
        j -= 1;
    }

    // Calculate that checksum.
    let mut i = 0;
    let mut res: usize = 0;
    for b in blocks.iter() {
        if *b != 0 {
            let id = b - FILE_ID_OFFSET;
            res += (i * id);
        }
        i += 1;
    }

    print!("{res}");
}

fn get_disk_layout(input: Vec<u8>) -> Vec<usize> {
    let mut blocks: Vec<usize> = Vec::new();
    let mut is_file = true;
    let mut id: usize = FILE_ID_OFFSET;
    for b in input {
        if b == LINE_FEED { break; }

        let n = b - CHAR_NUMBER_OFFSET;

        if is_file {
            // Add file id for all blocks occupied by its size (n).
            assert!(n > 0);
            for i in 0..n {
                blocks.push(id);
            }
            id += 1;
        } else {
            // Add '0' padding occupying free space (n) after the file.
            for i in 0..n {
                blocks.push(IS_FREE_BLOCK);
            }
        }
        is_file = !is_file;
    }

    blocks
}
