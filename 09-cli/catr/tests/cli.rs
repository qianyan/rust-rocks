use std::{fs, error::Error};

use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::{thread_rng, Rng, distributions::Alphanumeric};

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