use chrono::{prelude::*, Datelike, Duration};
use darth_tools::{ChronoTrait, DarthTools};

#[test]
fn test_new_date_utc_now() {
    let date = DarthTools::new_date_utc_now();
    assert_eq!(date.timezone(), Utc);
}

#[test]
fn test_new_date_local_now() {
    let date = DarthTools::new_date_local_now();
    let now = Local::now();
    assert_eq!(date.date_naive(), now.date_naive());
}

#[test]

fn test_new_date_utc_add_time_by_days() {
    let days = 1;
    let date = DarthTools::new_date_utc_now();
    let new_date = DarthTools::new_date_utc_add_time_by_days(days);

    println!("Original Date: {:?}", date);
    println!("New Date: {:?}", new_date);

    assert_eq!(date.day() + days as u32, new_date.day());
}

#[test]

fn test_new_date_local_add_time_by_days() {
    let days = 1;
    let date = DarthTools::new_date_local_now();
    let new_date = DarthTools::new_date_local_add_time_by_days(days);

    println!("Original Date: {:?}", date);
    println!("New Date: {:?}", new_date);

    assert_eq!(date.day() + days as u32, new_date.day());
}

#[test]
fn test_new_date_local_add_time_by_minutes() {
    let minutes = 1;
    let date = DarthTools::new_date_local_now();
    let new_date = DarthTools::new_date_local_add_time_by_minutes(minutes);
    assert_eq!(date.minute() + minutes as u32, new_date.minute());
}

#[test]
fn test_new_date_utc_add_time_by_hours() {
    let hours = 1;
    let date = DarthTools::new_date_utc_now();
    let new_date = DarthTools::new_date_utc_add_time_by_hours(hours);
    assert_eq!((date.hour() + hours as u32) % 24, new_date.hour());
}

#[test]
fn test_new_date_local_add_time_by_hours() {
    let hours = 1;
    let date = DarthTools::new_date_local_now();
    let new_date = DarthTools::new_date_local_add_time_by_hours(hours);
    assert_eq!((date.hour() + hours as u32) % 24, new_date.hour());
}

#[test]
fn test_new_date_utc_add_time_by_weeks() {
    let weeks = 1;
    let date = DarthTools::new_date_utc_now();
    let new_date = DarthTools::new_date_utc_add_time_by_weeks(weeks);
    assert_eq!((new_date - date).num_weeks(), weeks);
}

#[test]
fn test_new_date_local_add_time_by_weeks() {
    let weeks = 1;
    let date = DarthTools::new_date_local_now();
    let new_date = DarthTools::new_date_local_add_time_by_weeks(weeks);
    assert_eq!((new_date - date).num_weeks(), weeks);
}

#[test]
fn test_date_utc_is_expired() {
    let date = DarthTools::new_date_utc_now();
    let expiration_date = date - Duration::seconds(1);
    assert!(DarthTools::date_utc_is_expired(expiration_date));
}

#[test]
fn test_date_local_is_expired() {
    let date = DarthTools::new_date_local_now();
    let expiration_date = date - Duration::seconds(1);
    assert!(DarthTools::date_local_is_expired(expiration_date));
}

#[test]
fn test_duration_sec() {
    let time = 1;
    let duration = DarthTools::duration_sec(time);
    assert_eq!(duration.num_seconds(), time);
}

#[test]
fn test_duration_minutes() {
    let time = 1;
    let duration = DarthTools::duration_minutes(time);
    assert_eq!(duration.num_minutes(), time);
}

#[test]
fn test_duration_hours() {
    let time = 1;
    let duration = DarthTools::duration_hours(time);
    assert_eq!(duration.num_hours(), time);
}

#[test]
fn test_duration_days() {
    let time = 1;
    let duration = DarthTools::duration_days(time);
    assert_eq!(duration.num_days(), time);
}

#[test]
fn test_duration_weeks() {
    let time = 1;
    let duration = DarthTools::duration_weeks(time);
    assert_eq!(duration.num_weeks(), time);
}

#[test]
fn test_new_date_local_add_time_by_seconds() {
    let seconds = 1;
    let date = DarthTools::new_date_local_now();
    let new_date = DarthTools::new_date_local_add_time_by_seconds(seconds);
    assert!((new_date - date).num_seconds() >= seconds);
}

#[test]
fn test_new_date_utc_add_time_by_seconds() {
    let seconds = 1;
    let date = DarthTools::new_date_utc_now();
    let new_date = DarthTools::new_date_utc_add_time_by_seconds(seconds);
    assert!((new_date - date).num_seconds() >= seconds);
}
