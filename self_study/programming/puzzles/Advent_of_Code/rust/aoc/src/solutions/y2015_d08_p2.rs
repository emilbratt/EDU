pub fn main() {
    // Lets solve this in one go!
    let ans = std::fs::read_to_string("y2015_d08.in")
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            // Exploting the debug functionality using format!(),
            // this will expand the string for. :):)
            acc + format!("{:?}", line).len() - line.len()
        });
    print!("{ans}");
}
