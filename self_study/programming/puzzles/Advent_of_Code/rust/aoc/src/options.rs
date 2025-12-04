use std::{fs, env};

#[derive(Debug, Copy, Clone)]
pub enum Year {
    Year2015,
    Year2016,
    Year2017,
    Year2018,
    Year2019,
    Year2020,
    Year2021,
    Year2022,
    Year2023,
    Year2024,
    Year2025,
}

impl Year {
    fn new(n: u16) -> Self {
        match n {
            2015 => Self::Year2015,
            2016 => Self::Year2016,
            2017 => Self::Year2017,
            2018 => Self::Year2018,
            2019 => Self::Year2019,
            2020 => Self::Year2020,
            2021 => Self::Year2021,
            2022 => Self::Year2022,
            2023 => Self::Year2023,
            2024 => Self::Year2024,
            2025 => Self::Year2025,
            n => panic!("{n} is not a valid year"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
            n => panic!("{n} is not a valid day"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Part {
    Part1,
    Part2,
}

impl Part {
    fn new(part: u8) -> Self {
        match part {
            1 => Self::Part1,
            2 => Self::Part2,
            n => panic!("{n} is not a valid part"),
        }
    }
}

pub fn try_from_args() -> Option<(Year, Day, Part)> {
    let args: Vec<String> = env::args().collect();

    let (year, day, part) = match args.as_slice() {
        [_exe, y, d, p] =>  (y.as_str(), d.as_str(), p.as_str()),
        _ => return None,
    };

    let year = year.parse::<u16>().unwrap();
    let day = day.parse::<u8>().unwrap();
    let part = part.parse::<u8>().unwrap();

    Some((Year::new(year), Day::new(day), Part::new(part)))
}

pub fn try_from_file(path: &str) -> Option<Vec<(Year, Day, Part)>> {
    let f = match fs::read_to_string(path) {
        Err(_) => return None,
        Ok(f) => f,
    };

    let mut options: Vec<(Year, Day, Part)> = Vec::new();
    for line in f.lines() {
        if !line.starts_with('#') {
            let mut split = line.split(',');
            let year = split.next().unwrap().parse::<u16>().unwrap();
            let day = split.next().unwrap().parse::<u8>().unwrap();
            let part = split.next().unwrap().parse::<u8>().unwrap();
            let option = (Year::new(year), Day::new(day), Part::new(part));
            options.push(option);
        }
    }

    if options.is_empty() {
        println!("No options found in {path}");
        None
    } else {
        Some(options)
    }
}

pub fn get(year: u16, day: u8, part: u8) -> (Year, Day, Part) {
    (
        Year::new(year),
        Day::new(day),
        Part::new(part),
    )
}
