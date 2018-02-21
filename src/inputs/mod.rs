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

// We're relying on the CLI tests to test this, rather than trying to make ArgMatches for unit tests here.
impl Inputs {
    pub fn new(args: ArgMatches) -> Result<Inputs, &'static str> {
        let mut input_format_type = DateFormat::Dashes;

        let mut start: Option<Date<Utc>> = match args.value_of("start") {
            Some(value) => {
                let parsed_value = date_string_parser::ParsedDateString::new(value)?;
                input_format_type = parsed_value.format_type;
                Some(Utc.ymd(parsed_value.year, parsed_value.month, parsed_value.day))
            }
            None => None,
        };

        let mut end: Option<Date<Utc>> = match args.value_of("end") {
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

        let format_type = match args.value_of("format") {
            Some(value) => DateFormat::Custom(value.to_string()),
            None => input_format_type,
        };

        let today = if args.is_present("today") {
            Some(Utc::today())
        } else {
            None
        };

        if start.is_some() && end.is_some() && today.is_some() {
            return Err("--today cannot be used with both start and end specified");
        }

        // today overrides the start date, so
        // set end = start and start = today
        // if we have an offset with today, there should be no end date
        if today.is_some() {
            if !offset.is_some() {
                end = start;
            }

            start = today;
        }

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
