use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() {
    let outfile = "tests/expects/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn hello2() {
    let outfile = "tests/expects/hello2.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello")
        .arg("there")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello1n() {
    let outfile = "tests/expects/hello1.n.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("-n")
        .arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello2n() {
    let outfile = "tests/expects/hello2.n.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("-n")
        .arg("Hello")
        .arg("there")
        .assert()
        .success()
        .stdout(expected);
}
