use std::fs;

#[derive(Debug, Copy, Clone)]
pub enum Year {
    Year2025,
}

impl Year {
    fn new(n: u16) -> Self {
        match n {
            2025 => Self::Year2025,
            n => panic!("{n} is not a valid year"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Puzzle {
    P01,
    P02,
    P03,
    P04,
    P05,
    P06,
    P07,
    P08,
    P09,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
    P16,
    P17,
    P18,
    P19,
    P20,
}

impl Puzzle {
    fn new(n: u8) -> Self {
        match n {
            1  => Self::P01,
            2  => Self::P02,
            3  => Self::P03,
            4  => Self::P04,
            5  => Self::P05,
            6  => Self::P06,
            7  => Self::P07,
            8  => Self::P08,
            9  => Self::P09,
            10 => Self::P10,
            11 => Self::P11,
            12 => Self::P12,
            13 => Self::P13,
            14 => Self::P14,
            15 => Self::P15,
            16 => Self::P16,
            17 => Self::P17,
            18 => Self::P18,
            19 => Self::P19,
            20 => Self::P20,
            n => panic!("{n} is not a valid puzzle"),
        }
    }
}

pub fn try_from_file(path: &str) -> Option<Vec<(Year, Puzzle)>> {
    let f = match fs::read_to_string(path) {
        Err(_) => return None,
        Ok(f) => f,
    };

    let mut options: Vec<(Year, Puzzle)> = Vec::new();
    for line in f.lines() {
        if !line.starts_with('#') {
            let mut split = line.split(',');
            let year = split.next().unwrap().parse::<u16>().unwrap();
            let puzzle = split.next().unwrap().parse::<u8>().unwrap();
            let option = (Year::new(year), Puzzle::new(puzzle));
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

pub fn get(year: u16, day: u8) -> (Year, Puzzle) {
    (
        Year::new(year),
        Puzzle::new(day),
    )
}
