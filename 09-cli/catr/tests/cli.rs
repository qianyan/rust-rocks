use std::{error::Error, fs};

use assert_cmd::Command;
use predicates::{prelude::predicate, str::is_match};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

fn gen_bad_file() -> String {
    loop {
        let filename: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

type TestResult = Result<(), Box<dyn Error>>;
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn bustle() -> TestResult {
    run(&["-b", BUSTLE], format!("{}.out", BUSTLE).as_str())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], EMPTY)
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], format!("{}.out", FOX).as_str())
}

#[test]
fn spiders() -> TestResult {
    run(&["-n", FOX], format!("{}.out", FOX).as_str())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], format!("{}.stdin.out", BUSTLE).as_str())
}
