extern crate assert_cli;

#[test]
fn non_numeric_positive_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-p junk"])
    .fails()
    .and()
    .stderr().contains("parse")
    .unwrap();
}


#[test]
fn non_int_positive_offset() {
    assert_cli::Assert::command(&["target/debug/days_between", "20170101", "-p 1.1"])
    .fails()
    .and()
    .stderr().contains("parse")
    .unwrap();
}

// --- positive offset
// 0 offset
// negative int
// positive int
