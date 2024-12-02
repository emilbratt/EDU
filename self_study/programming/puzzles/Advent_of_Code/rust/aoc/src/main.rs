#![allow(unused)]
#![allow(non_snake_case)]

use std::time::Instant;

mod solutions;
mod options;

fn main() {
    let (year, day, part) = options::get(2024, 1, 2);
    let timer = true;

    match solutions::get(year, day, part) {
        None => println!("{:?} {:?} {:?} - not implemented", year, day, part),
        Some(solution) => {
            let instant = Instant::now();
            print!("{:?} {:?} {:?} | Result: ", year, day, part);
            solution();
            if timer {
                println!(" | Time: {} ms",instant.elapsed().as_millis());
            }
        }
    }
}
