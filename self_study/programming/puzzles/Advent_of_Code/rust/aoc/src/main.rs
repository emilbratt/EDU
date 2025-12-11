#![allow(non_snake_case)]
#![allow(unused)]

use std::{env, path::Path, time::Instant}; 

mod downloader;
mod options;
mod solutions;

const LATEST_YEAR: u16 = 2025;
const OPTIONS_IN: &str = "options.in"; // a csv list of year,day,part

fn main() {
    let mut options: Vec<(options::Year, options::Day, options::Part)> = Vec::new();

    if let Some(value) = options::try_from_args() {
        println!("Selecting puzzles from cli args: {:?}", value);
        options.push(value);
    } else if let Some(try_options) = options::try_from_file(OPTIONS_IN) {
        options = try_options;
        println!("Selecting puzzles from {OPTIONS_IN}");
        for (year, day, part) in options.iter() {
            println!(" -> {:?} {:?} {:?}", year, day, part)
        }
        println!();
    } else {
        println!("Selecting all puzzles");
        for year in 2015..=LATEST_YEAR {
            for day in 1..=25 {
                if year == 2025 && day > 12 {
                    continue;
                }
                for part in 1..=2 {
                    options.push(options::get(year, day, part));
                }
            }
        }
    }

    if !Path::new("session.in").exists() {
        panic!("Create file session.in and store session coockie for AoC in it..")
    }
    let session = std::fs::read_to_string("session.in").unwrap().lines().next().unwrap().to_string();
    let mut session: Option<String> = None;
    for (y,d, p) in options.iter() {
        if let None = session {
            session = Some(
                format!("session={}", std::fs::read_to_string("session.in").unwrap().lines().next().unwrap())
            )
        }

        if let Some(session) = &session {
            downloader::download(session, y.as_str(), d.as_str());
        } else {
            unreachable!();
        }

    }

    let instant = Instant::now();
    for option in options {
        let (year, day, part) = option;
        solve(year, day, part);
    }
    let elapsed = instant.elapsed().as_millis();

    println!("Total: {} ms", elapsed);
}

fn solve(year: options::Year, day: options::Day, part: options::Part) {
    match solutions::get(year, day, part) {
        None => {
            panic!("{:?} {:?} {:?} - not implemented", year, day, part);
        }
        Some(solution) => {
            let instant = Instant::now();
            print!("{:?} {:?} {:?} | Result: ", year, day, part);
            solution();
            let elapsed = instant.elapsed().as_micros();
            println!(" | Time: {} ms", elapsed as f64 / 1000_f64);
        }
    }
}
