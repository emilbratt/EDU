use std::fs;

pub fn main() {
    let input: String = fs::read_to_string("y2015_d02.in").unwrap();

    let mut res: i64 = 0;

    for line in input.lines() {

        let mut split = line.split("x");

        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let z = split.next().unwrap();

        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        let z = z.parse::<i64>().unwrap();

        let lwh = vec![
            x * y,
            y * z,
            z * x,
        ];

        let dimension: i64 = lwh.iter().map(|n| 2 * n).sum();

        let slack = lwh.iter().min().unwrap().to_owned();

        res += dimension + slack;
    }

    print!("{res}");
}
