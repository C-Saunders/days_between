extern crate chrono;
extern crate days_between;

use std::env;
use std::process;
use chrono::{Utc, TimeZone, Date};
use days_between::Inputs;

fn main() {
    let Inputs { start, end } = Inputs::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument parsing error: {}", err);
        process::exit(1);
    });
    let start = parse_date(&start);
    let end = parse_date(&end);
    
    println!("{}", end.signed_duration_since(start).num_days());
}

fn parse_date(s: &String) -> Date<Utc> {
    let y: i32 = *&s[0..4].parse().unwrap();
    let m: u32 = *&s[4..6].parse().unwrap();
    let d: u32 = *&s[6..8].parse().unwrap();
    Utc.ymd(y, m, d)
}
