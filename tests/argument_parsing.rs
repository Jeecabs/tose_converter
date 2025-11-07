use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_no_arguments_required() {
    // The tool should work with no arguments, reading psql table from stdin
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    let input = "  id  |  name  \n------+--------\n   1  | Alice  \n   2  | Bob    \n(2 rows)\n";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq("result[2]{id,name}:\n1,Alice\n2,Bob\n"));
}

#[test]
fn test_arguments_ignored() {
    // Even if arguments are provided, they should be ignored (for backward compat testing)
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    let input = "  id  |  name  \n------+--------\n   1  | Alice  \n(1 row)\n";
    // Providing args that would have been used in old version
    cmd.arg("users")
        .arg("id")
        .arg("name")
        .write_stdin(input)
        .assert()
        .success()
        // Output should still use "result" entity name from psql parsing, not "users" from args
        .stdout(predicate::eq("result[1]{id,name}:\n1,Alice\n"));
}

#[test]
fn test_empty_input_fails_gracefully() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.write_stdin("")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Empty input"));
}

#[test]
fn test_non_psql_input_fails() {
    // CSV input should fail since we only accept psql format now
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.write_stdin("id,name\n1,Alice\n2,Bob\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("no separator line found"));
}
