use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_check_valid_file() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("check")
        .arg("tests/tests/ok/primitives.mon")
        .assert()
        .success()
        .stdout(predicate::str::contains("Syntax OK"));
}

#[test]
fn test_check_invalid_missing_comma() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("check")
        .arg("tests/tests/bad/invalid_missing_comma.mon")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Parse error"));
}

#[test]
fn test_check_invalid_unclosed_object() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("check")
        .arg("tests/tests/bad/invalid_unclosed_object.mon")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Parse error"));
}

#[test]
fn test_check_nonexistent_file() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("check").arg("nonexistent.mon").assert().failure();
}

#[test]
fn test_compile_to_json() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/primitives.mon")
        .arg("--to")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"string\": \"hello world\""))
        .stdout(predicate::str::contains("\"number_int\": 42"));
}

#[test]
fn test_compile_to_yaml() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/primitives.mon")
        .arg("--to")
        .arg("yaml")
        .assert()
        .success()
        .stdout(predicate::str::contains("string: hello world"))
        .stdout(predicate::str::contains("number_int: 42"));
}

#[test]
#[ignore = "TOML format has limitations with null values - this is a known limitation"]
fn test_compile_to_toml() {
    // TOML serialization has limitations with certain types (e.g., null values)
    // Using a simpler test file for TOML
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/collections.mon")
        .arg("--to")
        .arg("toml")
        .assert()
        .success();
}

#[test]
fn test_compile_invalid_format() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/primitives.mon")
        .arg("--to")
        .arg("xml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported format"));
}

#[test]
fn test_compile_collections() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/collections.mon")
        .arg("--to")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("simple_array"))
        .stdout(predicate::str::contains("nested_object"));
}

#[test]
fn test_compile_structs() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/structs.mon")
        .arg("--to")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("valid_user"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn test_fmt_formats_correctly() {
    Command::cargo_bin("mon")
        .unwrap()
        .args(&["fmt", "tests/tests/ok/composition.mon"])
        .assert()
        .success()
        .stdout(predicate::str::contains("base_user"))
        .stdout(predicate::str::contains("base_permissions"));
}

#[test]
fn test_fmt_with_style() {
    Command::cargo_bin("mon")
        .unwrap()
        .args(&["fmt", "tests/tests/ok/composition.mon", "--style", "prettier"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Using prettier style"));
}

#[test]
fn test_fmt_check_unformatted() {
    Command::cargo_bin("mon")
        .unwrap()
        .args(&["fmt", "tests/tests/ok/composition.mon", "--check"])
        .assert()
        .failure(); // Should fail because file is not formatted
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Mycel Object Notation"))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("compile"))
        .stdout(predicate::str::contains("fmt"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("--version").assert().success().stdout(predicate::str::contains("mon"));
}

#[test]
fn test_error_reporting_quality() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("check")
        .arg("tests/tests/bad/invalid_missing_comma.mon")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Parse error"))
        .stderr(predicate::str::contains("expected"));
}

#[test]
fn test_compile_to_toml_with_null_removal() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/primitives.mon")
        .arg("--to")
        .arg("toml")
        .assert()
        .success()
        .stdout(predicate::str::contains("string = \"hello world\""))
        .stdout(predicate::str::contains("number_int = 42"))
        // Should NOT contain is_null field since null values are omitted
        .stdout(predicate::str::contains("is_null").not());
}

#[test]
fn test_compile_to_toml_with_null_value_replacement() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/primitives.mon")
        .arg("--to")
        .arg("toml")
        .arg("--toml-null-value")
        .arg("N/A")
        .assert()
        .success()
        .stdout(predicate::str::contains("is_null = \"N/A\""));
}

#[test]
fn test_compile_to_json_schema() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/structs.mon")
        .arg("--to")
        .arg("json-schema")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"object\""))
        .stdout(predicate::str::contains("\"properties\""))
        .stdout(predicate::str::contains("\"required\""));
}

#[test]
fn test_compile_json_schema_primitives() {
    let mut cmd = Command::cargo_bin("mon").unwrap();
    cmd.arg("compile")
        .arg("tests/tests/ok/primitives.mon")
        .arg("--to")
        .arg("json-schema")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"string\""))
        .stdout(predicate::str::contains("\"type\": \"number\""))
        .stdout(predicate::str::contains("\"type\": \"boolean\""));
}
