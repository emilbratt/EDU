use crate::options::{Day, Part, Year};

type Solution = Box<dyn Fn() + 'static>;

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

mod y2015_d06_p1;
mod y2015_d06_p2;

mod y2015_d07_p1;
mod y2015_d07_p2;

mod y2015_d08_p1;
mod y2015_d08_p2;

mod y2015_d09_p1;
mod y2015_d09_p2;

mod y2015_d10_p1;
mod y2015_d10_p2;

mod y2015_d11_p1;
mod y2015_d11_p2;

mod y2015_d12_p1;
mod y2015_d12_p2;

mod y2015_d13_p1;
mod y2015_d13_p2;

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
mod y2025_d01_p2;

mod y2025_d02_p1;
mod y2025_d02_p2;

mod y2025_d03_p1;
mod y2025_d03_p2;

mod y2025_d04_p1;
mod y2025_d04_p2;

mod y2025_d05_p1;
mod y2025_d05_p2;

mod y2025_d06_p1;
mod y2025_d06_p2;

mod y2025_d07_p1;
mod y2025_d07_p2;

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

        // --- Day 6: Probably a Fire Hazard ---
        (Year::Year2015, Day::Day06, Part::Part1) => y2015_d06_p1::main,
        (Year::Year2015, Day::Day06, Part::Part2) => y2015_d06_p2::main,

        // --- Day 7: Some Assembly Required ---
        (Year::Year2015, Day::Day07, Part::Part1) => y2015_d07_p1::main,
        (Year::Year2015, Day::Day07, Part::Part2) => y2015_d07_p2::main,

        // --- Day 8: Matchsticks ---
        (Year::Year2015, Day::Day08, Part::Part1) => y2015_d08_p1::main,
        (Year::Year2015, Day::Day08, Part::Part2) => y2015_d08_p2::main,

        // --- Day 9: All in a Single Night ---
        (Year::Year2015, Day::Day09, Part::Part1) => y2015_d09_p1::main,
        (Year::Year2015, Day::Day09, Part::Part2) => y2015_d09_p2::main,

        // --- Day 10: Elves Look, Elves Say ---
        (Year::Year2015, Day::Day10, Part::Part1) => y2015_d10_p1::main,
        (Year::Year2015, Day::Day10, Part::Part2) => y2015_d10_p2::main,

        // --- Day 11: Corporate Policy ---
        (Year::Year2015, Day::Day11, Part::Part1) => y2015_d11_p1::main,
        (Year::Year2015, Day::Day11, Part::Part2) => y2015_d11_p2::main,

        // --- Day 12: JSAbacusFramework.io ---
        (Year::Year2015, Day::Day12, Part::Part1) => y2015_d12_p1::main,
        (Year::Year2015, Day::Day12, Part::Part2) => y2015_d12_p2::main,

        // --- Day 13: Knights of the Dinner Table ---
        (Year::Year2015, Day::Day13, Part::Part1) => y2015_d13_p1::main,
        (Year::Year2015, Day::Day13, Part::Part2) => y2015_d13_p2::main,

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
        (Year::Year2025, Day::Day01, Part::Part2) => y2025_d01_p2::main,

        // --- Day 2: Gift Shop ---
        (Year::Year2025, Day::Day02, Part::Part1) => y2025_d02_p1::main,
        (Year::Year2025, Day::Day02, Part::Part2) => y2025_d02_p2::main,

        // --- Day 3: Lobby ---
        (Year::Year2025, Day::Day03, Part::Part1) => y2025_d03_p1::main,
        (Year::Year2025, Day::Day03, Part::Part2) => y2025_d03_p2::main,

        // --- Day 4: Printing Department ---
        (Year::Year2025, Day::Day04, Part::Part1) => y2025_d04_p1::main,
        (Year::Year2025, Day::Day04, Part::Part2) => y2025_d04_p2::main,

        // --- Day 5: Cafeteria ---
        (Year::Year2025, Day::Day05, Part::Part1) => y2025_d05_p1::main,
        (Year::Year2025, Day::Day05, Part::Part2) => y2025_d05_p2::main,

        // --- Day 6: Trash Compactor ---
        (Year::Year2025, Day::Day06, Part::Part1) => y2025_d06_p1::main,
        (Year::Year2025, Day::Day06, Part::Part2) => y2025_d06_p2::main,

        // --- Day 7: Laboratories ---
        (Year::Year2025, Day::Day07, Part::Part1) => y2025_d07_p1::main,
        (Year::Year2025, Day::Day07, Part::Part2) => y2025_d07_p2::main,

        _ => return None,
    };

    Some(Box::new(solution))
}
