use crate::options::{Year, Day, Part};

type Solution = Box<dyn Fn() + 'static>;

// I might write a macro to handle this in the future,
// but I like how structured this is.
pub fn get(year: Year, day: Day, part: Part) -> Option<Solution> {
    let solution = match (year, day, part) {

        // YEAR 2015
        // --- Day 1: Not Quite Lisp ---
        (Year::Year2015, Day::Day01, Part::Part1) => y2015d01p1::main,
        (Year::Year2015, Day::Day01, Part::Part2) => y2015d01p2::main,

        // --- Day 2: I Was Told There Would Be No Math ---
        (Year::Year2015, Day::Day02, Part::Part1) => y2015d02p1::main,
        (Year::Year2015, Day::Day02, Part::Part2) => y2015d02p2::main,

        // --- Day 3: Perfectly Spherical Houses in a Vacuum ---
        (Year::Year2015, Day::Day03, Part::Part1) => y2015d03p1::main,
        (Year::Year2015, Day::Day03, Part::Part2) => y2015d03p2::main,

        // --- Day 4: The Ideal Stocking Stuffer ---
        (Year::Year2015, Day::Day04, Part::Part1) => y2015d04p1::main,
        (Year::Year2015, Day::Day04, Part::Part2) => y2015d04p2::main,

        // --- Day 5: Doesn't He Have Intern-Elves For This? ---
        (Year::Year2015, Day::Day05, Part::Part1) => y2015d05p1::main,
        (Year::Year2015, Day::Day05, Part::Part2) => y2015d05p2::main,

        // YEAR 2024
        // --- Day 1: Historian Hysteria ---
        (Year::Year2024, Day::Day01, Part::Part1) => y2024d01p1::main,
        (Year::Year2024, Day::Day01, Part::Part2) => y2024d01p2::main,

        // --- Day 2: Red-Nosed Reports ---
        (Year::Year2024, Day::Day02, Part::Part1) => y2024d02p1::main,
        (Year::Year2024, Day::Day02, Part::Part2) => y2024d02p2::main,

        // --- Day 3: Mull It Over ---
        (Year::Year2024, Day::Day03, Part::Part1) => y2024d03p1::main,
        (Year::Year2024, Day::Day03, Part::Part2) => y2024d03p2::main,

        // --- Day 4: Ceres Search ---
        (Year::Year2024, Day::Day04, Part::Part1) => y2024d04p1::main,
        (Year::Year2024, Day::Day04, Part::Part2) => y2024d04p2::main,

        // --- Day 5: Print Queue ---
        (Year::Year2024, Day::Day05, Part::Part1) => y2024d05p1::main,
        (Year::Year2024, Day::Day05, Part::Part2) => y2024d05p2::main,

        (_, _, _) => return None,
    };

    Some(Box::new(solution))
}

// add solution files in ./solutions/ and define them as modules here..

// 2015
mod y2015d01p1;
mod y2015d01p2;

mod y2015d02p1;
mod y2015d02p2;

mod y2015d03p1;
mod y2015d03p2;

mod y2015d04p1;
mod y2015d04p2;

mod y2015d05p1;
mod y2015d05p2;

// 2024
mod y2024d01p1;
mod y2024d01p2;

mod y2024d02p1;
mod y2024d02p2;

mod y2024d03p1;
mod y2024d03p2;

mod y2024d04p1;
mod y2024d04p2;

mod y2024d05p1;
mod y2024d05p2;
