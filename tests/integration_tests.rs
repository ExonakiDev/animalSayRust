use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("animalsay")
    .expect("binary exists")
    .assert()
    .stdout(predicate::str::contains("Moose say Moo!"))
    .success();

    Ok(())
}

#[test]
fn fail_on_missing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("animalsay")
    .expect("File does not exist in directory")
    .args(&["-f", "/*.txt not present"])
    .assert()
    .failure();

    Ok(())
}