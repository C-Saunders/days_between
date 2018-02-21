extern crate assert_cli;
extern crate chrono;
use chrono::{Duration, Utc};

#[test]
fn non_numeric_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-o=junk"])
        .fails()
        .and()
        .stderr()
        .contains("parse")
        .unwrap();
}

#[test]
fn non_int_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-o=1.1"])
        .fails()
        .and()
        .stderr()
        .contains("parse")
        .unwrap();
}

#[test]
fn zero_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-o=0"])
        .succeeds()
        .and()
        .stdout()
        .is("20170101")
        .unwrap();
}

#[test]
fn negative_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20180209", "-o=-3"])
        .succeeds()
        .and()
        .stdout()
        .is("20180206")
        .unwrap();
}

#[test]
fn positive_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20180209", "-o=3"])
        .succeeds()
        .and()
        .stdout()
        .is("20180212")
        .unwrap();
}

#[test]
fn format_matches_start_date() {
    assert_cli::Assert::command(&["target/debug/days_between", "2018-02-09", "-o=3"])
        .succeeds()
        .and()
        .stdout()
        .is("2018-02-12")
        .unwrap();
}

#[test]
fn custom_format() {
    assert_cli::Assert::command(&["target/debug/days_between", "2018-02-09", "-o=3", "-f=%D"])
        .succeeds()
        .and()
        .stdout()
        .is("02/12/18")
        .unwrap();
}

#[test]
fn positive_offset_today() {
    assert_cli::Assert::command(&["target/debug/days_between", "--today", "-o=3"])
        .succeeds()
        .and()
        .stdout()
        .is(format!(
            "{}",
            Utc::today()
                .checked_add_signed(Duration::days(3))
                .unwrap()
                .format("%Y-%m-%d")
        ))
        .unwrap();
}

#[test]
fn negative_offset_today() {
    assert_cli::Assert::command(&["target/debug/days_between", "-t", "-o=-3"])
        .succeeds()
        .and()
        .stdout()
        .is(format!(
            "{}",
            Utc::today()
                .checked_sub_signed(Duration::days(3))
                .unwrap()
                .format("%Y-%m-%d")
        ))
        .unwrap();
}
