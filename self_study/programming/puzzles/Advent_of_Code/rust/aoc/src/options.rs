use std::{env, fs};

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
            _ => unreachable!(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Year2015 => "2015",
            Self::Year2016 => "2016",
            Self::Year2017 => "2017",
            Self::Year2018 => "2018",
            Self::Year2019 => "2019",
            Self::Year2020 => "2020",
            Self::Year2021 => "2021",
            Self::Year2022 => "2022",
            Self::Year2023 => "2023",
            Self::Year2024 => "2024",
            Self::Year2025 => "2025",
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
            _ => unreachable!(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Day01 => "01",
            Self::Day02 => "02",
            Self::Day03 => "03",
            Self::Day04 => "04",
            Self::Day05 => "05",
            Self::Day06 => "06",
            Self::Day07 => "07",
            Self::Day08 => "08",
            Self::Day09 => "09",
            Self::Day10 => "10",
            Self::Day11 => "11",
            Self::Day12 => "12",
            Self::Day13 => "13",
            Self::Day14 => "14",
            Self::Day15 => "15",
            Self::Day16 => "16",
            Self::Day17 => "17",
            Self::Day18 => "18",
            Self::Day19 => "19",
            Self::Day20 => "20",
            Self::Day21 => "21",
            Self::Day22 => "22",
            Self::Day23 => "23",
            Self::Day24 => "24",
            Self::Day25 => "25",
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
            _ => unreachable!(),
        }
    }
}

pub fn try_from_args() -> Vec<(Year, Day, Part)> {
    let mut options: Vec<(Year, Day, Part)> = Vec::new();

    let args = &env::args().collect::<Vec<String>>()[1..];
    match args {
        [y, d, p] =>  {
            let year = Year::new(y.parse::<u16>().unwrap());
            let day = Day::new(d.parse::<u8>().unwrap());
            let part = Part::new(p.parse::<u8>().unwrap());
            options.push((year, day, part));
        }
        [y, d] => {
            let year = Year::new(y.parse::<u16>().unwrap());
            let day = Day::new(d.parse::<u8>().unwrap());
            options.push( (year, day, Part::Part1) );
            options.push( (year, day, Part::Part2) );
        }
        [_] => {
            panic!("Pass at least a year and a day (passing part 1 or 2 is optional)");
        }
        _ => return options,
    };

    options
}

pub fn try_from_file(path: &str) -> Vec<(Year, Day, Part)> {
    let mut options: Vec<(Year, Day, Part)> = Vec::new();

    let f = match fs::read_to_string(path) {
        Err(_) => return options,
        Ok(f) => f,
    };

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

    options
}
