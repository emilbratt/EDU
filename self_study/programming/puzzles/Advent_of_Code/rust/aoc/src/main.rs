use std::{path::Path, time::Instant};

mod downloader;
mod options;
mod solutions;

const OPTIONS_IN: &str = "options.in"; // a csv list of year,day,part

fn main() {
    let mut options = options::try_from_args();
    if options.is_empty() {
        options = options::try_from_file(OPTIONS_IN);
    }

    let mut input_exists = true;
    for (y,d,_) in options.iter() {
        if !Path::new(&format!("y{}_d{}.in", y.as_str(), d.as_str())).exists() {
            input_exists = false;
        }
    }

    if !input_exists {
        let f = Path::new("session.in");
        if !f.exists() {
            panic!("Create file session.in and store session coockie for AoC in it..")
        }
        let session = std::fs::read_to_string(f).unwrap().lines().next().unwrap().to_string();
        let session = format!("session={}", session);
        for (y,d, _) in options.iter() {
            downloader::download(&session, y.as_str(), d.as_str());
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
