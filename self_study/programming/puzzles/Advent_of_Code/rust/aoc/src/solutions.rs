use crate::options::{Year, Day, Part};

type Solution = Box<dyn Fn() + 'static>;

pub fn get(year: Year, day: Day, part: Part) -> Option<Solution> {
    let solution = match (year, day, part) {
        // --- Day 1: Historian Hysteria ---
        (Year::Year2024, Day::Day01, Part::Part1) => y2024d01p1::main,
        (Year::Year2024, Day::Day01, Part::Part2) => y2024d01p2::main,

        // --- Day 2: Red-Nosed Reports ---
        (Year::Year2024, Day::Day02, Part::Part1) => y2024d02p1::main,
        (Year::Year2024, Day::Day02, Part::Part2) => y2024d02p2::main,

        // --- Day 3: Mull It Over ---
        (Year::Year2024, Day::Day03, Part::Part1) => y2024d03p1::main,
        (Year::Year2024, Day::Day03, Part::Part2) => y2024d03p2::main,

        (year, day, part) => return None,
    };

    Some(Box::new(solution))
}

// add solution files in ./solutions/ and define them as modules here..
mod y2024d01p1;
mod y2024d01p2;

mod y2024d02p1;
mod y2024d02p2;

mod y2024d03p1;
mod y2024d03p2;
