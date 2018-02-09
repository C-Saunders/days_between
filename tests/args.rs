extern crate assert_cli;

#[test]
fn fails_for_no_arguments() {
    assert_cli::Assert::command(&["target/debug/days_between"])
        .fails()
        .and()
        .stderr().contains("required arguments")
        .unwrap();
}

#[test]
fn fails_for_one_argument() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-01"])
        .fails()
        .and()
        .stderr().contains("have one of")
        .unwrap();
}

#[test]
fn fails_for_end_and_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-01", "2017-01-02", "-o 1"])
        .fails()
        .and()
        .stderr().contains("cannot be used with")
        .unwrap();
}

#[test]
fn fails_with_malformed_start_date() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017010", "20170102"])
        .fails()
        .and()
        .stderr().contains("parse date")
        .unwrap();
}

#[test]
#[should_panic]
fn panics_for_invalid_start_date() {
    assert_cli::Assert::command(&["target/debug/days_between", "2017-01-32", "20170102"]).unwrap();
}
