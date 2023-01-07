use chrono::{NaiveDate, Datelike, Weekday};
use counter::Counter;

fn main() {
    println!("{:?}", (1901..=2000).flat_map(|y| (1..=12).map(move |m| NaiveDate::from_ymd(y, m, 1).weekday())).collect::<Counter<_>>());
}
