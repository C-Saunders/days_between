extern crate assert_cli;
extern crate chrono;
use chrono::{Duration, Utc};

#[test]
fn list_end_after_start() {
    assert_cli::Assert::command(&[
        "target/debug/days_between",
        "2017-01-01",
        "2017-01-03",
        "-l",
    ]).succeeds()
        .and()
        .stdout()
        .contains("2017-01-01")
        .and()
        .stdout()
        .contains("2017-01-02")
        .and()
        .stdout()
        .contains("2017-01-03")
        .unwrap();
}

#[test]
fn list_end_equals_start() {
    assert_cli::Assert::command(&[
        "target/debug/days_between",
        "2017-01-01",
        "2017-01-01",
        "-l",
    ]).succeeds()
        .and()
        .stdout()
        .is("2017-01-01")
        .unwrap();
}

#[test]
fn list_end_before_start() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170103", "2017-01-01", "-l"])
        .succeeds()
        .and()
        .stdout()
        .contains("20170103")
        .and()
        .stdout()
        .contains("20170102")
        .and()
        .stdout()
        .contains("20170101")
        .unwrap();
}

#[test]
fn list_positive_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-01", "-o=2", "--list"])
        .succeeds()
        .and()
        .stdout()
        .contains("2017-01-01")
        .and()
        .stdout()
        .contains("2017-01-02")
        .and()
        .stdout()
        .contains("2017-01-03")
        .unwrap();
}

#[test]
fn list_zero_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-01", "-o=0", "--list"])
        .succeeds()
        .and()
        .stdout()
        .is("2017-01-01")
        .unwrap();
}

#[test]
fn list_negative_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170103", "-o=-2", "-l"])
        .succeeds()
        .and()
        .stdout()
        .contains("20170103")
        .and()
        .stdout()
        .contains("20170102")
        .and()
        .stdout()
        .contains("20170101")
        .unwrap();
}

#[test]
fn list_custom_format() {
    assert_cli::Assert::command(&[
        "target/debug/days_between",
        "20170103",
        "-o=-2",
        "-l",
        "-f=%v",
    ]).succeeds()
        .and()
        .stdout()
        .contains("3-Jan-2017")
        .and()
        .stdout()
        .contains("2-Jan-2017")
        .and()
        .stdout()
        .contains("1-Jan-2017")
        .unwrap();
}

// TODO: See if we can use a macro or something to generate these date strings and clean this up
#[test]
fn end_today_list() {
    let end_date = format!(
        "{}",
        Utc::today()
            .checked_add_signed(Duration::days(2))
            .unwrap()
            .format("%Y-%m-%d")
    );

    assert_cli::Assert::command(&["target/debug/days_between", "--today", &end_date, "-l"])
        .succeeds()
        .and()
        .stdout()
        .contains(format!("{}", Utc::today().format("%Y-%m-%d")))
        .and()
        .stdout()
        .contains(format!(
            "{}",
            Utc::today()
                .checked_add_signed(Duration::days(1))
                .unwrap()
                .format("%Y-%m-%d")
        ))
        .and()
        .stdout()
        .contains(end_date)
        .unwrap();
}

#[test]
fn offset_today_list() {
    assert_cli::Assert::command(&["target/debug/days_between", "--today", "-o=2", "-l"])
        .succeeds()
        .and()
        .stdout()
        .contains(format!("{}", Utc::today().format("%Y-%m-%d")))
        .and()
        .stdout()
        .contains(format!(
            "{}",
            Utc::today()
                .checked_add_signed(Duration::days(1))
                .unwrap()
                .format("%Y-%m-%d")
        ))
        .and()
        .stdout()
        .contains(format!(
            "{}",
            Utc::today()
                .checked_add_signed(Duration::days(2))
                .unwrap()
                .format("%Y-%m-%d")
        ))
        .unwrap();
}
