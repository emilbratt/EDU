use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime, NaiveDate, NaiveTime, Utc};
use chrono_tz::Tz;

pub fn run() {
    println!("{:?}", naivedate("2024-04-27"));

    println!("{:?}", utc_now());

    println!("{:?}", local_now());

    println!("{:?}", convert_naivedatetime_to_utc_datetime("2024-04-27T09:00:00"));

    println!("{:?}", from_utc_to_local(utc_now()));

    println!("{:?}", yyyymmdd_to_datetime("2024-04-27"));

    println!("{:?}", local_utc_offset_in_seconds(local_now()));

    println!("{:?}", london_datetime_now());

    println!("{:?}", oslo_date_now());

    println!("{:?}", stockholm_time_now());
}

fn naivedate(dt: &str) -> NaiveDate {
    NaiveDate::parse_from_str(dt, "%Y-%m-%d").unwrap()
}

fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

fn local_now() -> DateTime<Local> {
    Local::now()
}

fn convert_naivedatetime_to_utc_datetime(dt: &str) -> DateTime<Utc> {
    let dt_naive = NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").unwrap();

    Utc.from_utc_datetime(&dt_naive)
}

fn from_utc_to_local(dt: DateTime<Utc>) -> DateTime<Local> {
    let dt_local: DateTime<Local> = DateTime::from(dt);

    dt_local
}

fn yyyymmdd_to_datetime(dt: &str) -> DateTime<Utc>{
    let dt_format = format!("{}T{}", dt, "00:00:00");

    let dt_naive = NaiveDateTime::parse_from_str(&dt_format, "%Y-%m-%dT%H:%M:%S").unwrap();

    let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_naive);

    dt_utc
}

fn local_utc_offset_in_seconds(dt: DateTime<Local>) -> i32 {
    dt.offset().local_minus_utc()
}

fn london_datetime_now() -> DateTime<Tz> {
    let dt: DateTime<Tz>  = Local::now().with_timezone(&chrono_tz::Europe::London);

    dt
}

fn oslo_date_now() -> NaiveDate {
    let dt: DateTime<Tz> = Local::now().with_timezone(&chrono_tz::Europe::Oslo);

    dt.date_naive()
}

fn stockholm_time_now() -> NaiveTime {
    let dt: DateTime<Tz> = Local::now().with_timezone(&chrono_tz::Europe::Stockholm);

    dt.time()
}
