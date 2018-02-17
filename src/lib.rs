extern crate chrono;
extern crate clap;
extern crate regex;
pub mod inputs;
pub mod calculate;

pub enum DateFormat {
    NoDashes,
    Dashes,
    Custom(String),
}
