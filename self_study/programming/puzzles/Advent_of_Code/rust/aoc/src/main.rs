#![allow(non_snake_case)]
#![allow(unused)]

use std::time::Instant;

mod solutions;
mod options;

const DEBUG: bool = false;

const YEAR: options::Year = options::Year::Year2024;
const DAY: options::Day   = options::Day::Day05;
const PART: options::Part = options::Part::Part2;

const USE_CONST_OPTIONS: bool = true;

fn main() {
    if USE_CONST_OPTIONS {
        run(YEAR, DAY, PART);
    } else {
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
