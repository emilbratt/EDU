pub fn main() {
    let input = std::fs::read_to_string("y2015_d08.in").unwrap();
    let mut total = 0usize;
    let mut mem = 0usize;
    for l in input.lines() {
        total += l.len();
        let bytes = l.as_bytes();
        let mut i = 1;
        while i < bytes.len()-1 {
            mem += 1;
            match bytes[i] {
                b'\\' => {
                    match bytes[i+1] {
                        b'\\' => i += 2,
                        b'"' => i += 2,
                        b'x' => i += 4, // any 2 characters after x is part of the ascii code.
                        byte => unreachable!("Did not expect character '{}'", byte as char) // we only consider the above rules..
                    }
                }
                byte => i += 1,
            }
        }
    }
    print!("{}", total - mem);
}
