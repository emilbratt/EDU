use std::fs;

#[derive(Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    let input: Vec<u8> = fs::read("y2015d03.in").unwrap();

    let mut points: Vec<Point<i64>> = Vec::new();

    // start point (always counted..).
    let mut point = Point { x: 0_i64, y: 0_i64 };

    points.push(point);

    for b in input {
        match b {
            60  => point.x -= 1, // left '<'
            62  => point.x += 1, // right '>'
            94  => point.y += 1, // up '^'
            118 => point.y -= 1, // down 'v'
            _ => panic!(),
        }

        if !points.contains(&point) {
            points.push(point);
        }
    }

    let res = points.len();

    print!("{res}");
}
