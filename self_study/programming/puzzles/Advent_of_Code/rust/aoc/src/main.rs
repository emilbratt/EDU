#![allow(unused)]
#![allow(non_snake_case)]

use std::time::Instant;

mod solutions;
mod options;

pub const DEBUG: bool = false;

fn main() {
    let instant = Instant::now();

    for year in 2015..=2024 {
        for day in 1..=25 {
            for part in 1..=2 {
                let (y, d, p) = options::get(year, day, part);
                run(y, d, p);
            }
        }
    }

    let elapsed = instant.elapsed().as_millis();

    println!("Total: {} ms", elapsed);
}

fn run(year: options::Year, day: options::Day, part: options::Part) {
    match solutions::get(year, day, part) {
        None => {
            if DEBUG {
                println!("{:?} {:?} {:?} - not implemented", year, day, part);
            }
        }
        Some(solution) => {
            let instant = Instant::now();

            print!("{:?} {:?} {:?} | Result: ", year, day, part);

            solution();

            let elapsed = instant.elapsed().as_millis();

            println!(" | Time: {} ms", elapsed);
        }
    }
}
