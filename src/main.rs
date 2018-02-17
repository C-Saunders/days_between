extern crate clap;
extern crate days_between;

use clap::{App, Arg};
use std::process;
use days_between::{calculate, inputs};

fn main() {
    let args = App::new("DaysBetween")
        .version("0.4.0")
        .author("Charlie S. <charlieasaunders@gmail.com>")
        .about("A command line utility for working with date ranges.")
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
        .arg(Arg::with_name("list")
            .help("Print dates in the calculated range, newline delimited.")
            .short("l")
            .long("list")
            .required(false))
        .arg(Arg::with_name("format")
            .help("Output date format. See https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html")
            .short("f")
            .long("format")
            .takes_value(true)
            .required(false))
        .get_matches();

    let args = inputs::Inputs::new(args).unwrap_or_else(|err| {
        eprintln!("Argument parsing error: {}", err);
        process::exit(1);
    });

    calculate::print_output(args)
}
