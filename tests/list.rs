extern crate assert_cli;

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
