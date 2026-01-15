#![allow(dead_code)]
#![allow(unused)]

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

use std::io::BufReader;

struct Chunk {
    chunk_type: [u8; 4],
    data: Vec<u8>,
}


pub fn load() {
    let _ = BufReader::new(std::fs::File::open("images/2x2.png").unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        load();
    }
}
