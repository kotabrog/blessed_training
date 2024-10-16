use chrono::{naive, FixedOffset, Local, Month, NaiveDate, TimeDelta, Timelike};

fn main() {
    let month = "January".parse::<Month>().unwrap();
    println!("{:?}", month);
    let month: Month = Month::try_from(8).unwrap();
    println!("{:?}", month);
    let month: Month = 8.try_into().unwrap();
    println!("{:?}", month);

    let time_delta = TimeDelta::new(-10, 10).unwrap();
    println!("time_delta: {:?}", time_delta);
    println!("time_delta subsec_nanos: {:?}", time_delta.subsec_nanos());

    let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
    let time_delta = TimeDelta::days(10) + TimeDelta::hours(10);
    let new_date = date.checked_add_signed(time_delta).unwrap();
    println!("date: {:?}", date);
    println!("new_date: {:?}", new_date); 
    let date = NaiveDate::from_ymd_opt(2021, 1, 20).unwrap();
    let new_date = date.checked_sub_signed(time_delta).unwrap();
    println!("date: {:?}", date);
    println!("new_date: {:?}", new_date);

    let date1 = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
    let date2 = NaiveDate::from_ymd_opt(2022, 1, 10).unwrap();
    println!("data1, date2: {:?}", date1.years_since(date2));
    println!("data2, date1: {:?}", date2.years_since(date1));
    println!("data1 < date2: {:?}", date1 < date2);

    let offset = FixedOffset::east_opt(3600).unwrap();
    println!("offset: {:?}", offset);
    println!("local_minus_utc: {:?}", offset.local_minus_utc());
    println!("utc_minus_local: {:?}", offset.utc_minus_local());

    let datetime = Local::now();
    println!("datetime: {:?}", datetime);

    let datetime2 = datetime.with_timezone(&FixedOffset::east_opt(3600).unwrap());
    println!("datetime2: {:?}", datetime2);

    let datetime3 = datetime.fixed_offset();
    println!("datetime3: {:?}", datetime3);

    let datetime4 = datetime.to_utc();
    println!("datetime4: {:?}", datetime4);

    let naive_datetime = datetime.naive_utc();
    println!("naive_datetime: {:?}", naive_datetime);
    let naive_datetime = datetime.naive_local();
    println!("naive_datetime: {:?}", naive_datetime);

    println!("datetime: {:?}", datetime.to_rfc2822());

    println!("datetime timelike: {:?}", datetime.hour());
}
