use crate::options::{Year, Puzzle};

type Solution = Box<dyn Fn() + 'static>;

pub fn get(year: Year, puzzle: Puzzle) -> Option<Solution> {
    let solution = match (year, puzzle) {

        // YEAR 2025
        // --- Puzzle 1: Length limits on messaging platforms ---
        (Year::Year2025, Puzzle::P01) => y2025_1::main,

        // --- Puzzle 2: Detecting gravitational waves ---
        (Year::Year2025, Puzzle::P02) => y2025_2::main,

        _ => return None,
    };

    Some(Box::new(solution))
}

// Solution files go in ./solutions/y{YYYY}_{N}.rs and are defind as modules here.
mod y2025_1;
mod y2025_2;
