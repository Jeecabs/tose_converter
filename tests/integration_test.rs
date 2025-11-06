use assert_cmd::Command;
use predicates::prelude::*;

/// Test helper to run the tose_converter binary with given args and stdin
fn run_converter(entity: &str, fields: &[&str], input: &str) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.arg(entity);
    for field in fields {
        cmd.arg(field);
    }
    cmd.write_stdin(input).assert()
}

#[test]
fn test_spec_example_7_1_simple() {
    // TOSE Specification Example 7.1: Simple order items
    // Input: orderItems sku qty price
    // Data: A1,2,9.99\nB2,1,14.50\n
    // Expected: orderItems[2]{sku,qty,price}:\nA1,2,9.99\nB2,1,14.50\n

    let input = "A1,2,9.99\nB2,1,14.50\n";
    let expected = "orderItems[2]{sku,qty,price}:\nA1,2,9.99\nB2,1,14.50\n";

    run_converter("orderItems", &["sku", "qty", "price"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_spec_example_7_2_complex() {
    // TOSE Specification Example 7.2: Complex notes with NULL, quotes, commas
    // Input: notes id title body
    // Data: 1,"Note ""A""",\n2,"""Comma, Inc.""",A simple note.\n
    // Expected: notes[2]{id,title,body}:\n[data unchanged]

    let input = "1,\"Note \"\"A\"\"\",\n2,\"\"\"Comma, Inc.\"\"\",A simple note.\n";
    let expected = "notes[2]{id,title,body}:\n1,\"Note \"\"A\"\"\",\n2,\"\"\"Comma, Inc.\"\"\",A simple note.\n";

    run_converter("notes", &["id", "title", "body"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_empty_input() {
    // Empty input should produce a header with count of 0 and no data
    let input = "";
    let expected = "entity[0]{}:\n";

    run_converter("entity", &[], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_single_row_no_trailing_newline() {
    // Single row without trailing newline should still count as 1 row
    let input = "data";
    let expected = "test[1]{col}:\ndata";

    run_converter("test", &["col"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_single_row_with_trailing_newline() {
    let input = "data\n";
    let expected = "test[1]{col}:\ndata\n";

    run_converter("test", &["col"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_multiple_fields() {
    let input = "1,2,3,4,5\n";
    let expected = "nums[1]{a,b,c,d,e}:\n1,2,3,4,5\n";

    run_converter("nums", &["a", "b", "c", "d", "e"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_empty_field_list() {
    // No fields specified should result in empty braces {}
    let input = "row1\nrow2\n";
    let expected = "data[2]{}:\nrow1\nrow2\n";

    run_converter("data", &[], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_preserves_null_values() {
    // NULL values represented as empty fields (consecutive commas)
    let input = "1,,3\n4,5,\n";
    let expected = "test[2]{a,b,c}:\n1,,3\n4,5,\n";

    run_converter("test", &["a", "b", "c"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_preserves_quoted_commas() {
    // CSV fields with commas must be quoted - we preserve them exactly
    let input = "1,\"Hello, World\",3\n";
    let expected = "test[1]{a,b,c}:\n1,\"Hello, World\",3\n";

    run_converter("test", &["a", "b", "c"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_preserves_quoted_quotes() {
    // Doubled quotes inside quoted fields (RFC 4180)
    let input = "1,\"Quote \"\"test\"\"\",3\n";
    let expected = "test[1]{a,b,c}:\n1,\"Quote \"\"test\"\"\",3\n";

    run_converter("test", &["a", "b", "c"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_preserves_newlines_in_quoted_fields() {
    // Newlines inside quoted fields should be preserved
    // Note: The converter counts physical lines (by \n), not logical CSV records
    // So a quoted field with a newline will count as 2 rows
    let input = "1,\"Line1\nLine2\",3\n";
    let expected = "test[2]{a,b,c}:\n1,\"Line1\nLine2\",3\n";

    run_converter("test", &["a", "b", "c"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_mixed_empty_and_data_lines() {
    // Empty lines should not be counted
    let input = "row1\n\nrow2\n\nrow3\n";
    let expected = "test[3]{col}:\nrow1\n\nrow2\n\nrow3\n";

    run_converter("test", &["col"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_whitespace_only_lines_not_counted() {
    // Lines with only whitespace should not be counted as data rows
    let input = "row1\n   \nrow2\n\t\n";
    let expected = "test[2]{col}:\nrow1\n   \nrow2\n\t\n";

    run_converter("test", &["col"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_entity_name_with_underscores() {
    let input = "data\n";
    let expected = "order_items[1]{sku}:\ndata\n";

    run_converter("order_items", &["sku"], input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_entity_name_camel_case() {
    let input = "data\n";
    let expected = "orderItems[1]{sku}:\ndata\n";

    run_converter("orderItems", &["sku"], input)
        .success()
        .stdout(predicate::eq(expected));
}
