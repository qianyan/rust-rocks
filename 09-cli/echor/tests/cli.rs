use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn fails_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn ok_with_args() {
    Command::cargo_bin("echor")
        .unwrap()
        .arg("hello")
        .assert()
        .success();
}

#[test]
fn echo_hello_world() {
    let outfile = "tests/fixtures/hello_world.txt";
    run(&["Hello World"], outfile);
}

#[test]
fn echo_hello_x_world() {
    let outfile = "tests/fixtures/hello_world.spaces.txt";
    run(&["Hello", "World"], outfile);
}

#[test]
fn echo_hello_x_world_no_newline() {
    let outfile = "tests/fixtures/hello_world.spaces.n.txt";
    run(&["-n", "Hello", "World"], outfile);
}

#[test]
fn echo_hello_world_no_newline() {
    let outfile = "tests/fixtures/hello_world.n.txt";
    run(&["Hello World", "-n"], outfile);
}

fn run(args: &[&str], expected_file: &str) {
    let expected = fs::read_to_string(expected_file).unwrap();
    Command::cargo_bin("echor")
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}
