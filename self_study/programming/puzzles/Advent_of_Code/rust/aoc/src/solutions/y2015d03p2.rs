use std::fs;

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    let input: Vec<u8> = fs::read("y2015d03.in").unwrap();

    let mut points: HashSet<Point<i64>> = HashSet::new();

    let mut point_santa = Point { x: 0_i64, y: 0_i64 };
    let mut point_robo = Point { x: 0_i64, y: 0_i64 };
    
    // Start point is always counted.
    points.insert(point_santa);

    let mut santa = true;

    for b in input {
        let (x, y) = match b {
            60  => (-1, 0), // left '<'
            62  => (1, 0),  // right '>'
            94  => (0, 1),  // up '^'
            118 => (0, -1), // down 'v'
            _ => panic!(),
        };

        let cur_point: Point<i64> = match santa {
            true => {
                point_santa.x += x;
                point_santa.y += y;

                point_santa
            }
            false => {
                point_robo.x += x;
                point_robo.y += y;

                point_robo
            }
        };

        santa = !santa;

        points.insert(cur_point);
    }

    let res = points.len();

    print!("{res}");
}
