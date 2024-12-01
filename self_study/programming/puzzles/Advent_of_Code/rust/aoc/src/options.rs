// use std::fmt;

#[derive(Debug)]
pub enum Year {
    Year2024,
}

impl Year {
    fn new(n: u16) -> Self {
        match n {
            2024 => Self::Year2024,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Day {
    fn new(n: u8) -> Self {
        match n {
            1  => Self::Day01,
            2  => Self::Day02,
            3  => Self::Day03,
            4  => Self::Day04,
            5  => Self::Day05,
            6  => Self::Day06,
            7  => Self::Day07,
            8  => Self::Day08,
            9  => Self::Day09,
            10 => Self::Day10,
            11 => Self::Day11,
            12 => Self::Day12,
            13 => Self::Day13,
            14 => Self::Day14,
            15 => Self::Day15,
            16 => Self::Day16,
            17 => Self::Day17,
            18 => Self::Day18,
            19 => Self::Day19,
            20 => Self::Day20,
            21 => Self::Day21,
            22 => Self::Day22,
            23 => Self::Day23,
            24 => Self::Day24,
            25 => Self::Day25,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl Part {
    fn new(part: u8) -> Self {
        match part {
            1 => Self::Part1,
            2 => Self::Part2,
            _ => panic!(),
        }
    }
}

pub fn get(year: u16, day: u8, part: u8) -> (Year, Day, Part) {
    (
        Year::new(year),
        Day::new(day),
        Part::new(part),
    )
}
