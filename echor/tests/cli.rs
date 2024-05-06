use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}

fn runs(args: &[&str], expected_file: &str) -> TestResult {
    let expect = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expect);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    runs(&["Hello there"], "tests/expects/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    runs(&["Hello", "there"], "tests/expects/hello2.txt")
}

#[test]
fn hello1n() -> TestResult {
    runs(&["-n", "Hello there"], "tests/expects/hello1.n.txt")
}

#[test]
fn hello2n() -> TestResult {
    runs(&["-n", "Hello", "there"], "tests/expects/hello2.n.txt")
}
