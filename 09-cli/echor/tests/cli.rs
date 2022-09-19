use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;
// Box indicates that the error will live inside a kind of pointer where the memory is dynamically allocated on the heap rather than the stack.
// dyn indicates that the method calls on the std::error:Error trait are dynamically dispatched.

#[test]
fn fails_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn ok_with_args() -> TestResult {
    Command::cargo_bin("echor")?.arg("hello").assert().success();
    Ok(())
}

#[test]
fn echo_hello_world() -> TestResult {
    let outfile = "tests/fixtures/hello_world.txt";
    run(&["Hello World"], outfile)
}

#[test]
fn echo_hello_x_world() -> TestResult {
    let outfile = "tests/fixtures/hello_world.spaces.txt";
    run(&["Hello", "World"], outfile)
}

#[test]
fn echo_hello_x_world_no_newline() -> TestResult {
    let outfile = "tests/fixtures/hello_world.spaces.n.txt";
    run(&["-n", "Hello", "World"], outfile)
}

#[test]
fn echo_hello_world_no_newline() -> TestResult {
    let outfile = "tests/fixtures/hello_world.n.txt";
    run(&["Hello World", "-n"], outfile)
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).unwrap();
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
