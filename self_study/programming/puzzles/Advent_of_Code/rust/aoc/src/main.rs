#![allow(unused)]
#![allow(non_snake_case)]

use std::time::Instant;

mod solutions;
mod options;

fn main() {
    let (year, day, part) = options::get(2024, 1, 1);

    match solutions::get(year, day, part) {
        None => println!("{:?} {:?} {:?} - not implemented", year, day, part),
        Some(solution) => {
            let instant = Instant::now();
            print!("{:?} {:?} {:?} | ", year, day, part);
            solution();
            println!(" | {} ms",instant.elapsed().as_millis());
        }
    }
}
