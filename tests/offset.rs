extern crate assert_cli;

#[test]
fn non_numeric_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-o=junk"])
    .fails()
    .and()
    .stderr().contains("parse")
    .unwrap();
}


#[test]
fn non_int_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-o=1.1"])
    .fails()
    .and()
    .stderr().contains("parse")
    .unwrap();
}

#[test]
fn zero_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-o=0"])
    .succeeds()
    .and()
    .stdout().is("20170101")
    .unwrap();
}

#[test]
fn negative_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20180209", "-o=-3"])
    .succeeds()
    .and()
    .stdout().is("20180206")
    .unwrap();
}

#[test]
fn positive_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20180209", "-o=3"])
    .succeeds()
    .and()
    .stdout().is("20180212")
    .unwrap();
}
