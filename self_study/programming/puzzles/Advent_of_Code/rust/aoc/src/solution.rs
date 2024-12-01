use std::time::{Duration, Instant};
use std::thread;

use crate::options::{Year, Day, Part};

mod year2024;

type Solution = Box<dyn Fn() + 'static>;

pub fn run(year: &Year, day: &Day, part: &Part) {

    let solution: Solution = match (year, day, part) {
        (Year::Year2024, Day::Day01, Part::Part1) => Box::new(year2024::day01_part1::main),
        (year, day, part) => panic!("{}", format!("{:?} {:?} {:?} is not implemented..", year, day, part)),
    };

    let instant = Instant::now();
    solution();
    println!("{:?} {:?} {:?} | {} ms", year, day, part, instant.elapsed().as_millis());
}
