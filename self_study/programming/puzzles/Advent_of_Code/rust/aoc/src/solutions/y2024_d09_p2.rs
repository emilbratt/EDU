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

    let mut blocks: Vec<usize> = get_disk_layout(input);

    // Move files.
    let mut j = blocks.len() - 1;
    while j > 0 {
        while j > 0 && blocks[j] == IS_FREE_BLOCK {
            j -= 1;
        }

        let id = blocks[j];
        let mut f_move: Vec<usize> = Vec::new();
        while j > 0 && blocks[j] == id {
            f_move.push(j);
            j -= 1;
        }

        let mut f_free: Vec<usize> = Vec::new();
        let mut k = 0;
        while k < j {
            f_free.clear();
            while k < blocks.len() && blocks[k] != IS_FREE_BLOCK {
                k += 1;
            }
            while k < blocks.len() && blocks[k] == IS_FREE_BLOCK {
                f_free.push(k);
                k += 1;
            }
            if f_move.len() <= f_free.len() && k <= j + 1 {
                for i in 0..f_move.len() {
                    let m = f_move[i];
                    let o = f_free[i];
                    blocks.swap(m, o);
                }
                break;
            }
        }
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
            assert!(n > 0 && n <= 9);
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
