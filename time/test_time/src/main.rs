use std::str::FromStr;

use time::Month;
use time::convert::{Microsecond, Second};
use time::Duration;
use time::Date;
use time::Time;
use time::UtcOffset;
use time::OffsetDateTime;
use time::ext::NumericalDuration;

fn main() {
    let month: Month = 4.try_into().unwrap();
    println!("month: {:?}", month);
    let month = Month::from_str("April").unwrap();
    println!("month: {:?}", month);

    let microsecond = Microsecond::per(Second);
    println!("microsecond: {:?}", microsecond);

    let duration = Duration::seconds(1);
    println!("duration: {:?}", duration);
    let duration = Duration::default();
    println!("duration: {:?}", duration);

    let duration = Duration::SECOND / 2;
    println!("duration: {:?}", duration);
    println!("duration: {}", duration);

    let date = Date::from_calendar_date(2024, Month::try_from(2).unwrap(), 29).unwrap();
    println!("day: {:?}", date);
    let date = date.replace_year(2023);
    println!("day: {:?}", date);
    let time1 = Time::from_hms(12, 0, 0).unwrap();
    let time2 = Time::from_hms(13, 0, 0).unwrap();
    let duration = time2 - time1;
    println!("duration: {:?}", duration);
    let duration = time1 - time2;
    println!("duration: {:?}", duration);

    let offset = UtcOffset::from_hms(26, 0, 0);
    println!("offset: {:?}", offset);
    let offset = UtcOffset::from_hms(24, -20, 10);
    println!("offset: {:?}", offset);

    let offset_date_time = OffsetDateTime::now_utc();
    println!("offset_date_time: {:?}", offset_date_time);
    let offset = UtcOffset::local_offset_at(offset_date_time);
    println!("offset: {:?}", offset);
    let offset_date_time = OffsetDateTime::new_in_offset(offset_date_time.date(), offset_date_time.time(), UtcOffset::from_hms(1, 0, 0).unwrap());
    println!("offset_date_time: {:?}", offset_date_time);
    let offset = UtcOffset::local_offset_at(offset_date_time);
    println!("offset: {:?}", offset);

    let offset_date_time = OffsetDateTime::now_utc();
    println!("offset_date_time: {:?}", offset_date_time);
    let offset_date_time = OffsetDateTime::now_local();
    println!("offset_date_time: {:?}", offset_date_time);

    let offset_date_time = OffsetDateTime::from_unix_timestamp_nanos(0);
    println!("offset_date_time: {:?}", offset_date_time);

    let duration = 1.seconds();
    println!("duration: {:?}", duration);
}
