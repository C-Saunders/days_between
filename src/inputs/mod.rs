extern crate chrono;
extern crate clap;
extern crate regex;

mod date_string_parser;

use clap::ArgMatches;
use chrono::{Date, TimeZone, Utc};
use DateFormat;

pub struct Inputs {
    pub start: Option<Date<Utc>>,
    pub end: Option<Date<Utc>>,
    pub offset: Option<i64>,
    pub format_type: DateFormat,
    pub list_output: bool,
}

// Blatant duplication here. Not sure how to get rid of it nicely yet, due to the error handling.
// We're relying on the CLI tests to test this, rather than trying to make ArgMatches for unit tests here.
impl Inputs {
    pub fn new(args: ArgMatches) -> Result<Inputs, &'static str> {
        let format_type;

        let start: Option<Date<Utc>> = match args.value_of("start") {
            Some(value) => {
                let parsed_value = date_string_parser::ParsedDateString::new(value)?;
                format_type = parsed_value.format_type;
                Some(Utc.ymd(parsed_value.year, parsed_value.month, parsed_value.day))
            }
            None => return Err("Missing start date"),
        };

        let end: Option<Date<Utc>> = match args.value_of("end") {
            Some(value) => {
                let parsed_value = date_string_parser::ParsedDateString::new(value)?;
                Some(Utc.ymd(parsed_value.year, parsed_value.month, parsed_value.day))
            }
            None => None,
        };

        let offset: Option<i64> = match args.value_of("offset") {
            Some(value) => match value.parse::<i64>() {
                Ok(parsed_value) => Some(parsed_value),
                _ => return Err("Failed to parse offset"),
            },
            None => None,
        };

        if end.is_none() && offset.is_none() {
            return Err("Must have one of [end-date, offset].");
        }

        Ok(Inputs {
            start,
            end,
            offset,
            format_type,
            list_output: args.is_present("list"),
        })
    }
}
