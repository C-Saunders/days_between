extern crate assert_cli;

#[test]
fn non_numeric_negative_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-m junk"])
    .fails()
    .and()
    .stderr().contains("parse")
    .unwrap();
}


#[test]
fn non_int_negative_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-m 1.1"])
    .fails()
    .and()
    .stderr().contains("parse")
    .unwrap();
}

// 0 offset
// negative int
// positive int
