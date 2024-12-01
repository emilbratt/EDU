#![allow(unused)]

mod solution;
mod options;

fn main() {
    let (year, day, part) = options::get(2024, 1, 1);

    solution::run(&year, &day, &part);
}
