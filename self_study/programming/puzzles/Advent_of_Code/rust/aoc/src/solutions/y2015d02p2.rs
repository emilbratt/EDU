use std::fs;

pub fn main() {
    let input: String = fs::read_to_string("y2015d02.in").unwrap();

    let mut res: i64 = 0;

    for line in input.lines() {

        let mut split = line.split("x");

        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let z = split.next().unwrap();

        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        let z = z.parse::<i64>().unwrap();

        let mut lwh = vec![x, y, z];
        lwh.sort();
        lwh.reverse();

        let a = lwh.pop().unwrap();
        let b = lwh.pop().unwrap();
        let c = lwh.pop().unwrap();

        let ribbon = (a+a+b+b) + (a*b*c);

        res += ribbon;
    }

    print!("{res}");
}
