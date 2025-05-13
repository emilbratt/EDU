use std::fs;

use chrono::{DateTime, NaiveDateTime};

const INPUT: &str = "y2025_4.in";

pub fn main() {
    let input_str: String = fs::read_to_string(INPUT).unwrap();

    let mut lines = input_str.lines();

    // Specifiers: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    let fmt = "%b %d, %Y, %H:%M";

    let mut res = 0;
    while let (Some(departure), Some(arrival)) = (lines.next(), lines.next()) {

        let (tz_str, dt_str) = extract_tz_and_dt(departure);
        let departure_at = extract_local_datetime(fmt, tz_str, dt_str);

        let (tz_str, dt_str) = extract_tz_and_dt(arrival);
        let arrive_at = extract_local_datetime(fmt, tz_str, dt_str);

        let minutes = (arrive_at - departure_at).num_minutes();

        res += minutes;

        // Every 3rd line is empty, or if last (None), we break.
        if lines.next().is_none() { break }
    }
    assert_eq!(16451, res);
    print!("{res}");
}

fn extract_tz_and_dt<'a>(s: &'a str) -> (&'a str, &'a str) {
    let s = if s.starts_with("Departure: ") {
        s.strip_prefix("Departure: ").unwrap()
    } else if  s.starts_with("Arrival:   ") {
        s.strip_prefix("Arrival:   ").unwrap()
    } else {
        panic!();
    };

    let v: Vec<&'a str> = s.splitn(2, ' ').collect();

    (v[0], v[1].trim_start())
}

fn extract_local_datetime(fmt: &str, tz_str: &str, dt_str: &str) -> DateTime<chrono_tz::Tz> {
    let dt = NaiveDateTime::parse_from_str(dt_str, fmt).unwrap();
    let tz = tz_str.parse::<chrono_tz::Tz>().unwrap();

    dt.and_local_timezone(tz).unwrap()
}
