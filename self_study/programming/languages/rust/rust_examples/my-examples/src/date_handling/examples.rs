use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime, NaiveDate, NaiveTime, Utc, Duration, Timelike};
use chrono_tz::{
    Tz,
    Europe,
};

pub fn run() {
    println!("{:?}", naive_date());

    println!("{:?}", utc_now());

    println!("{:?}", local_now());

    println!("{:?}", naive_datetime_to_utc_datetime());

    println!("{:?}", utc_to_local());

    println!("{:?}", yyyymmdd_to_datetime());

    println!("{:?}", local_utc_offset_in_seconds(local_now()));

    println!("{:?}", london_datetime_now());

    println!("{:?}", oslo_date_now());

    println!("{:?}", stockholm_time_now());

    println!("{:?}", cest_to_cet_ambiguity());

    println!("{:?}", local_with_timezone_to_utc());

    println!("{:?}", utc_to_dt_with_timezone());

    println!("{:?}", custom_time());

    println!("{:?}", seconds_until_next_hour());
}

fn naive_date() -> NaiveDate {
    let dt = "2024-04-27";

    NaiveDate::parse_from_str(dt, "%Y-%m-%d").unwrap()
}

fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

fn local_now() -> DateTime<Local> {
    Local::now()
}

fn naive_datetime_to_utc_datetime() -> DateTime<Utc> {
    let dt = "2024-04-27T09:00:00";
    let fmt = "%Y-%m-%dT%H:%M:%S";

    let dt_naive = NaiveDateTime::parse_from_str(dt, fmt).unwrap();

    Utc.from_utc_datetime(&dt_naive)
}

fn utc_to_local() -> DateTime<Local> {
    let utc: DateTime<Utc> = Utc::now();

    let dt_local: DateTime<Local> = DateTime::from(utc);

    dt_local
}

fn yyyymmdd_to_datetime() -> DateTime<Utc>{
    let dt = "2024-04-27";

    let dt_format = format!("{}T{}", dt, "00:00:00");

    let dt_naive = NaiveDateTime::parse_from_str(&dt_format, "%Y-%m-%dT%H:%M:%S").unwrap();

    let dt_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_naive);

    dt_utc
}

fn local_utc_offset_in_seconds(dt: DateTime<Local>) -> i32 {
    dt.offset().local_minus_utc()
}

fn london_datetime_now() -> DateTime<Tz> {
    let dt: DateTime<Tz> = Utc::now().with_timezone(&chrono_tz::Europe::London);

    dt
}

fn oslo_date_now() -> NaiveDate {
    let dt: DateTime<Tz> = Utc::now().with_timezone(&chrono_tz::Europe::Oslo);

    dt.date_naive()
}

fn stockholm_time_now() -> NaiveTime {
    let dt: DateTime<Tz> = Utc::now().with_timezone(&chrono_tz::Europe::Stockholm);

    dt.time()
}

fn cest_to_cet_ambiguity() -> DateTime<Tz> {
    // This is one hour before we turned clock 1 hour back, e.g. moving from CEST to CET.
    let dt: DateTime<Tz> = chrono_tz::Europe::Oslo.with_ymd_and_hms(2022, 10, 30, 1, 0, 0).unwrap();
    // NOTE: setting (2022, 10, 30, 2, 0, 0) (e.g. 02:00) will panic because of ambiguity.

    let add_one_hours = dt + Duration::hours(1);
    let add_two_hours = dt + Duration::hours(2);

    assert_eq!(
        add_one_hours.hour(),
        add_two_hours.hour(),
    );

    dt
}

fn local_with_timezone_to_utc() -> DateTime<Utc> {
    let l: DateTime<Tz> = Local::now().with_timezone(&Europe::Oslo);

    l.to_utc()
}

fn utc_to_dt_with_timezone() -> DateTime<Tz> {
    let utc = Utc::now();

    utc.with_timezone(&Europe::Oslo)
}

fn custom_time() -> DateTime<Utc> {
    // +1 hour, minut = 0, seconds = 0
    let dt = (Utc::now() + Duration::hours(1))
        .with_minute(0).unwrap()
        .with_second(0).unwrap();

    dt
}

fn seconds_until_next_hour() -> i64 {
    let now = Utc::now();

    let next_hour = (Utc::now() + Duration::hours(1))
        .with_minute(0).unwrap()
        .with_second(0).unwrap();

    let s = (next_hour - now).num_seconds();

    s
}
