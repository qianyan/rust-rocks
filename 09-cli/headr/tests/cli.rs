use std::{error::Error, fs};

use assert_cmd::Command;

const PRG: &str = "headr";

type TestResult = Result<(), Box<dyn Error>>;
#[test]
fn print_the_first_10_lines_by_default() -> TestResult {
    let expected = fs::read_to_string("tests/expected/ten.txt.out")?;
    Command::cargo_bin(PRG)?
        .args(["tests/inputs/ten.txt"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
