use crate::options::{Year, Day, Part};

type Solution = Box<dyn Fn() + 'static>;

pub fn get(year: Year, day: Day, part: Part) -> Option<Solution> {

    let solution = match (year, day, part) {
        (Year::Year2024, Day::Day01, Part::Part1) => y2024d01p01::main,
        (year, day, part) => return None,
    };

    Some(Box::new(solution))
}

// add solution files as a module here..
mod y2024d01p01;
