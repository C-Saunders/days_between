extern crate clap;
extern crate days_between;

use clap::{App, Arg};
use std::process;
use days_between::{inputs, calculate};

fn main() {
    let args = App::new("DaysBetween")
        .version("0.2.0")
        .author("Charlie S. <charlieasaunders@gmail.com>")
        .about("A utility for calculating the number of days between two dates or a date offset from a start date.")
        .arg(Arg::with_name("start")
            .help("The start date for the calculation, formatted YYYYMMDD or YYYY-MM-DD.")
            .index(1)
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("end")
            .help("The end date for the calculation.")
            .short("e")
            .long("end-date")
            .conflicts_with(&"offset")
            .takes_value(true)
            .required(false)
            .index(2))
        .arg(Arg::with_name("offset")
            .help("Calculate the date this many days offset the start date. Positive or negative integer.")
            .short("o")
            .long("offset")
            .conflicts_with(&"end")
            .takes_value(true)
            .required(false))
        .get_matches();

    let args = inputs::Inputs::new(args).unwrap_or_else(|err| {
        eprintln!("Argument parsing error: {}", err);
        process::exit(1);
    });

    calculate::print_output(&args)
}
