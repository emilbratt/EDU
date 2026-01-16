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

// just to print out the hex values..
fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len());
    for b in bytes {
        let fmt = format!("{:X}", b);
        s.push_str( format!("{:0>2} ", fmt).as_str() );
    }
    println!("{}", s);
    s
}

pub fn load() -> io::Result<()> {
    let mut signature_buf = [0u8; 8];
    let mut chunk_buf = [0u8; 4];
    let f = File::open("images/2x2.png").unwrap();
    let mut rdr = BufReader::new(f);
    let n = rdr.read(&mut signature_buf)?;
    assert_eq!(n, 8);
    assert_eq!(PNG_SIGNATURE, signature_buf);
    hex(&signature_buf);

    while let Ok(n) = rdr.read(&mut chunk_buf) {
        hex(&chunk_buf);
        break;
    }

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
