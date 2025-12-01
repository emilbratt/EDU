use crate::options::{Year, Day, Part};

type Solution = Box<dyn Fn() + 'static>;

// I might write a macro to handle this in the future,
// but I like how structured this is.
pub fn get(year: Year, day: Day, part: Part) -> Option<Solution> {
    let solution = match (year, day, part) {

        // YEAR 2015
        // --- Day 1: Not Quite Lisp ---
        (Year::Year2015, Day::Day01, Part::Part1) => y2015_d01_p1::main,
        (Year::Year2015, Day::Day01, Part::Part2) => y2015_d01_p2::main,

        // --- Day 2: I Was Told There Would Be No Math ---
        (Year::Year2015, Day::Day02, Part::Part1) => y2015_d02_p1::main,
        (Year::Year2015, Day::Day02, Part::Part2) => y2015_d02_p2::main,

        // --- Day 3: Perfectly Spherical Houses in a Vacuum ---
        (Year::Year2015, Day::Day03, Part::Part1) => y2015_d03_p1::main,
        (Year::Year2015, Day::Day03, Part::Part2) => y2015_d03_p2::main,

        // --- Day 4: The Ideal Stocking Stuffer ---
        (Year::Year2015, Day::Day04, Part::Part1) => y2015_d04_p1::main,
        (Year::Year2015, Day::Day04, Part::Part2) => y2015_d04_p2::main,

        // --- Day 5: Doesn't He Have Intern-Elves For This? ---
        (Year::Year2015, Day::Day05, Part::Part1) => y2015_d05_p1::main,
        (Year::Year2015, Day::Day05, Part::Part2) => y2015_d05_p2::main,

        // YEAR 2023
        // --- Day 1: Trebuchet?! ---
        (Year::Year2023, Day::Day01, Part::Part1) => y2023_d01_p1::main,
        (Year::Year2023, Day::Day01, Part::Part2) => y2023_d01_p2::main,

        // YEAR 2024
        // --- Day 1: Historian Hysteria ---
        (Year::Year2024, Day::Day01, Part::Part1) => y2024_d01_p1::main,
        (Year::Year2024, Day::Day01, Part::Part2) => y2024_d01_p2::main,

        // --- Day 2: Red-Nosed Reports ---
        (Year::Year2024, Day::Day02, Part::Part1) => y2024_d02_p1::main,
        (Year::Year2024, Day::Day02, Part::Part2) => y2024_d02_p2::main,

        // --- Day 3: Mull It Over ---
        (Year::Year2024, Day::Day03, Part::Part1) => y2024_d03_p1::main,
        (Year::Year2024, Day::Day03, Part::Part2) => y2024_d03_p2::main,

        // --- Day 4: Ceres Search ---
        (Year::Year2024, Day::Day04, Part::Part1) => y2024_d04_p1::main,
        (Year::Year2024, Day::Day04, Part::Part2) => y2024_d04_p2::main,

        // --- Day 5: Print Queue ---
        (Year::Year2024, Day::Day05, Part::Part1) => y2024_d05_p1::main,
        (Year::Year2024, Day::Day05, Part::Part2) => y2024_d05_p2::main,

        // --- Day 6: Guard Gallivant ---
        (Year::Year2024, Day::Day06, Part::Part1) => y2024_d06_p1::main,
        (Year::Year2024, Day::Day06, Part::Part2) => y2024_d06_p2::main,

        // --- Day 7: Bridge Repair ---
        (Year::Year2024, Day::Day07, Part::Part1) => y2024_d07_p1::main,
        (Year::Year2024, Day::Day07, Part::Part2) => y2024_d07_p2::main,

        // --- Day 8: Resonant Collinearity ---
        (Year::Year2024, Day::Day08, Part::Part1) => y2024_d08_p1::main,
        (Year::Year2024, Day::Day08, Part::Part2) => y2024_d08_p2::main,

        // --- Day 9: Disk Fragmenter ---
        (Year::Year2024, Day::Day09, Part::Part1) => y2024_d09_p1::main,
        (Year::Year2024, Day::Day09, Part::Part2) => y2024_d09_p2::main,

        // YEAR 2025
        // --- Day 1: Secret Entrance ---
        (Year::Year2025, Day::Day01, Part::Part1) => y2025_d01_p1::main,
        // (Year::Year2024, Day::Day01, Part::Part2) => y2024_d01_p2::main,

        _ => return None,
    };

    Some(Box::new(solution))
}

// Solution files go in ./solutions/y{YYYY}_d{DD}_p{P}.rs and are defind as modules here.

// 2015
mod y2015_d01_p1;
mod y2015_d01_p2;

mod y2015_d02_p1;
mod y2015_d02_p2;

mod y2015_d03_p1;
mod y2015_d03_p2;

mod y2015_d04_p1;
mod y2015_d04_p2;

mod y2015_d05_p1;
mod y2015_d05_p2;

// 2023
mod y2023_d01_p1;
mod y2023_d01_p2;

// 2024
mod y2024_d01_p1;
mod y2024_d01_p2;

mod y2024_d02_p1;
mod y2024_d02_p2;

mod y2024_d03_p1;
mod y2024_d03_p2;

mod y2024_d04_p1;
mod y2024_d04_p2;

mod y2024_d05_p1;
mod y2024_d05_p2;

mod y2024_d06_p1;
mod y2024_d06_p2;

mod y2024_d07_p1;
mod y2024_d07_p2;

mod y2024_d08_p1;
mod y2024_d08_p2;

mod y2024_d09_p1;
mod y2024_d09_p2;

// 2025
mod y2025_d01_p1;
