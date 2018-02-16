extern crate assert_cli;

#[test]
fn fails_with_malformed_end_date() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "2017011"])
        .fails()
        .and()
        .stderr()
        .contains("parse date")
        .unwrap();
}

#[test]
#[should_panic]
fn panics_for_invalid_end_date() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-30", "2017-01-32"])
        .unwrap();
}

#[test]
fn end_before_start() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-03", "2017-01-01"])
        .succeeds()
        .and()
        .stdout()
        .is("-2")
        .unwrap();
}

#[test]
fn end_equals_start() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-01", "2017-01-01"])
        .succeeds()
        .and()
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn end_after_start() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-01", "2017-01-05"])
        .succeeds()
        .and()
        .stdout()
        .is("4")
        .unwrap();
}
