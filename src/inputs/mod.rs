extern crate chrono;
extern crate clap;
extern crate regex;

mod date_string_parser;

use clap::ArgMatches;
use chrono::{Utc, TimeZone, Date};

pub struct Inputs {
    pub start: Option<Date<Utc>>,
    pub end: Option<Date<Utc>>,
    pub plus_days: Option<i32>,
    pub minus_days: Option<i32>,
}

// Blatant duplication here. Not sure how to get rid of it nicely yet, due to the error handling.
// We're relying on the CLI tests to test this, rather than trying to make ArgMatches for unit tests here.
impl Inputs {
    pub fn new(args: ArgMatches) -> Result<Inputs, &'static str> {
        let start: Option<Date<Utc>> = match args.value_of("start") {
            Some(value) => {
                let parsed_value = date_string_parser::ParsedDateString::new(value)?;
                Some(Utc.ymd(parsed_value.year, parsed_value.month, parsed_value.day))
            },
            None => return Err("Missing start date"),
        };

        let end: Option<Date<Utc>> = match args.value_of("end") {
            Some(value) => {
                let parsed_value = date_string_parser::ParsedDateString::new(value)?;
                Some(Utc.ymd(parsed_value.year, parsed_value.month, parsed_value.day))
            },
            None => None,
        };

        let plus_days: Option<i32> = match args.value_of("plus") {
            Some(value) => {
                match value.parse::<i32>() {
                    Ok(parsed_value) => Some(parsed_value),
                    _ => return Err("Failed to parse integer offset"),
                }
            },
            None => None
        };

        let minus_days: Option<i32> = match args.value_of("minus") {
            Some(value) => {
                match value.parse::<i32>() {
                    Ok(parsed_value) => Some(parsed_value),
                    _ => return Err("Failed to parse integer offset"),
                }
            },
            None => None,
        };

        if end.is_none() && plus_days.is_none() && minus_days.is_none() {
            return Err("Must have one of [end-date, plus-days, minus-days].")
        }

        Ok(Inputs { start, end, plus_days, minus_days })
    }
}
