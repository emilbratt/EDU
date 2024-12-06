use std::fs;

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    let input: Vec<u8> = fs::read("y2015_d03.in").unwrap();

    let mut points: HashSet<Point<i64>> = HashSet::new();

    let mut point = Point { x: 0_i64, y: 0_i64 };
    
    // Start point is always counted.
    points.insert(point);

    for b in input {
        match b {
            60  => point.x -= 1, // left '<'
            62  => point.x += 1, // right '>'
            94  => point.y += 1, // up '^'
            118 => point.y -= 1, // down 'v'
            _ => panic!(),
        }

        points.insert(point);
    }

    let res = points.len();

    print!("{res}");
}
