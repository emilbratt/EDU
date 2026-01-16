#![allow(dead_code)]
#![allow(unused)]

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read}
};

struct Chunk {
    chunk_type: [u8; 4],
    data: Vec<u8>,
}

pub fn load() -> io::Result<()> {
    let mut buf = [0u8; 8];
    let f = File::open("images/2x2.png").unwrap();
    let mut rdr = BufReader::new(f);
    let n = rdr.read(&mut buf)?;
    assert_eq!(n, 8);
    assert_eq!(PNG_SIGNATURE, buf);
    println!("{:?}", buf);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        load();
    }
}
