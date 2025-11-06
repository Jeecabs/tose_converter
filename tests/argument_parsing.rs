use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_no_arguments_exits_with_error() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("Error: Missing required arguments."));
}

#[test]
fn test_only_entity_name_empty_field_list() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("entity")
        .write_stdin("row1\nrow2\n")
        .assert()
        .success()
        .stdout(predicate::eq("entity[2]{}:\nrow1\nrow2\n"));
}

#[test]
fn test_entity_plus_single_field() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("users")
        .arg("id")
        .write_stdin("1\n2\n")
        .assert()
        .success()
        .stdout(predicate::eq("users[2]{id}:\n1\n2\n"));
}

#[test]
fn test_entity_plus_multiple_fields() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("users")
        .arg("id")
        .arg("name")
        .arg("email")
        .write_stdin("1,Alice,alice@example.com\n")
        .assert()
        .success()
        .stdout(predicate::eq("users[1]{id,name,email}:\n1,Alice,alice@example.com\n"));
}

#[test]
fn test_entity_name_with_underscores() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("order_items")
        .arg("sku")
        .write_stdin("A1\n")
        .assert()
        .success()
        .stdout(predicate::eq("order_items[1]{sku}:\nA1\n"));
}

#[test]
fn test_entity_name_camel_case() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("orderItems")
        .arg("sku")
        .write_stdin("A1\n")
        .assert()
        .success()
        .stdout(predicate::eq("orderItems[1]{sku}:\nA1\n"));
}

#[test]
fn test_field_names_with_underscores() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("users")
        .arg("user_id")
        .arg("first_name")
        .arg("last_name")
        .write_stdin("1,John,Doe\n")
        .assert()
        .success()
        .stdout(predicate::eq("users[1]{user_id,first_name,last_name}:\n1,John,Doe\n"));
}

#[test]
fn test_field_names_camel_case() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("users")
        .arg("userId")
        .arg("firstName")
        .arg("lastName")
        .write_stdin("1,John,Doe\n")
        .assert()
        .success()
        .stdout(predicate::eq("users[1]{userId,firstName,lastName}:\n1,John,Doe\n"));
}

#[test]
fn test_many_fields() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("data")
        .arg("a")
        .arg("b")
        .arg("c")
        .arg("d")
        .arg("e")
        .arg("f")
        .arg("g")
        .arg("h")
        .arg("i")
        .arg("j")
        .write_stdin("1,2,3,4,5,6,7,8,9,10\n")
        .assert()
        .success()
        .stdout(predicate::eq("data[1]{a,b,c,d,e,f,g,h,i,j}:\n1,2,3,4,5,6,7,8,9,10\n"));
}

#[test]
fn test_numeric_entity_name() {
    // While not recommended, numeric entity names should work
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("123")
        .arg("col")
        .write_stdin("data\n")
        .assert()
        .success()
        .stdout(predicate::eq("123[1]{col}:\ndata\n"));
}

#[test]
fn test_entity_name_with_numbers() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("table2024")
        .arg("id")
        .write_stdin("1\n")
        .assert()
        .success()
        .stdout(predicate::eq("table2024[1]{id}:\n1\n"));
}

#[test]
fn test_single_character_entity() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("x")
        .arg("y")
        .write_stdin("data\n")
        .assert()
        .success()
        .stdout(predicate::eq("x[1]{y}:\ndata\n"));
}

#[test]
fn test_single_character_fields() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("data")
        .arg("a")
        .arg("b")
        .arg("c")
        .write_stdin("1,2,3\n")
        .assert()
        .success()
        .stdout(predicate::eq("data[1]{a,b,c}:\n1,2,3\n"));
}

#[test]
fn test_very_long_entity_name() {
    let long_name = "very_long_entity_name_with_many_characters_that_goes_on_and_on";
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg(long_name)
        .arg("id")
        .write_stdin("1\n")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(format!("{}[1]{{id}}:", long_name)));
}

#[test]
fn test_very_long_field_name() {
    let long_field = "very_long_field_name_with_many_characters";
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg("test")
        .arg(long_field)
        .write_stdin("data\n")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(format!("test[1]{{{}}}:", long_field)));
}

#[test]
fn test_usage_message_format() {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("<ENTITY_NAME>"))
        .stderr(predicate::str::contains("FIELD"));
}
