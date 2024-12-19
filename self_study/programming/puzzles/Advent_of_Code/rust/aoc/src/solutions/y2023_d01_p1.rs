const INPUT: &str = "y2023_d01.in";

pub fn main() {
    let input: String = std::fs::read_to_string(INPUT).unwrap();

    let mut res: i64 = 0;

    for line in input.lines() {
        let mut first: Option<i64> = None;
        let mut last: Option<i64> = None;

        for c in line.chars() {
            if c.is_numeric() {
                if first.is_none() {
                    first = Some(c.to_digit(10).unwrap() as i64);
                } else {
                    last = Some(c.to_digit(10).unwrap() as i64);
                }
            }
        }

        match (first, last) {
            ( Some(f), Some(l) ) => res += (f * 10 + l),

            // edge case for lines holding only one dgit meaning last will equal the first.
            ( Some(f_and_l), None ) => res += (f_and_l * 10 + f_and_l),

            _ => panic!(),
        }
    }

    print!("{res}");
}
