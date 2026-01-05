pub fn main() {
    let mut input: Vec<u8> = std::fs::read("y2015_d12.in").unwrap();
    
    let mut ans = 0;
    let mut cur = String::new();
    for b in input {
        if b == b'-' || b.is_ascii_digit() {
            cur.push(b as char);
        } else {
            if !cur.is_empty() {
                ans += cur.parse::<i32>().unwrap();
                cur.clear();
            }
        }
    }

    print!("{ans}");
}
