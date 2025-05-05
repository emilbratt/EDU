#![allow(unused)]

use std::time::Instant;

mod solutions;
mod options;

const DEBUG: bool = true;
const OPTIONS: &str = "options.in"; // a csv list of year,day,part

fn main() {
    let options = match options::try_from_file(OPTIONS) {
        Some(options) => {
            println!("Selecting puzzles from {OPTIONS}");
            options
        }
        None => {
            println!("Selecting all puzzles");
            let mut options: Vec<(options::Year, options::Puzzle)> = Vec::new();
            for year in 2025..=2025 {
                for puzzle in 1..=20 {
                    options.push(options::get(year, puzzle));
                }
            }

            options
        }
    };

    let instant = Instant::now();

    for (year, puzzle) in options {
        solve(year, puzzle);
    }

    let elapsed = instant.elapsed().as_millis();

    println!("Total: {} ms", elapsed);
}

fn solve(year: options::Year, puzzle: options::Puzzle) {
    match solutions::get(year, puzzle) {
        None => {
            if DEBUG {
                println!("{:?} {:?} - not implemented", year, puzzle);
            }
        }
        Some(solution) => {
            let instant = Instant::now();

            print!("{:?} {:?} | Result: ", year, puzzle);

            solution();

            let elapsed = instant.elapsed().as_micros();

            println!(" | Time: {} ms", elapsed as f64 / 1000_f64);
        }
    }
}
