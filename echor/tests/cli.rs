use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // The ? at the end is used to unpack the Result and return the error
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = std::fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
:
#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").arg("world");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = std::fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
