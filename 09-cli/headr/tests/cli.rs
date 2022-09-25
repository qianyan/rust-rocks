use std::{error::Error, fs};

use assert_cmd::Command;
use predicates::prelude::predicate;

const PRG: &str = "headr";
#[test]
fn test_parse_positive_int() {
    let res = headr::parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = headr::parse_positive_int("nonInteger");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "nonInteger".to_string());

    let res = headr::parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

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
