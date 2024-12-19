#![allow(non_snake_case)]
#![allow(unused)]

use std::time::Instant;

mod solutions;
mod options;

const DEBUG: bool = false;
const OPTIONS_IN: &str = "options.in"; // a csv list of year,day,part

fn main() {
    let options: Vec<(options::Year, options::Day, options::Part)>;

    options = match options::try_from_file(OPTIONS_IN) {
        Some(options) => {
            println!("Selecting puzzles from {OPTIONS_IN}");
            for &(year, day, part) in options.iter() {
                println!(" -> {:?} {:?} {:?}", year, day, part)
            }
            println!();
            options
        }
        None => {
            println!("Selecting all puzzles");
            let mut options: Vec<(options::Year, options::Day, options::Part)> = Vec::new();
            for year in 2015..=2024 {
                for day in 1..=25 {
                    for part in 1..=2 {
                        options.push(options::get(year, day, part));
                    }
                }
            }

            options
        }
    };

    let instant = Instant::now();

    for option in options {
        let (year, day, part) = option;
        solve(year, day, part);
    }

    let elapsed = instant.elapsed().as_millis();

    println!("Total: {} ms", elapsed);
}

fn solve(year: options::Year, day: options::Day, part: options::Part) {
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

            let elapsed = instant.elapsed().as_micros();

            println!(" | Time: {} ms", elapsed as f64 / 1000_f64);
        }
    }
}
