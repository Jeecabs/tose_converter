use assert_cmd::Command;
use predicates::prelude::*;

/// Test helper to run the tose_converter binary with psql table input
fn run_converter(input: &str) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("tose_converter").unwrap();
    cmd.write_stdin(input).assert()
}

#[test]
fn test_simple_table() {
    let input = "  sku  |  qty  | price\n-------+-------+-------\n   A1  |   2   | 9.99 \n   B2  |   1   | 14.50\n(2 rows)\n";
    let expected = "result[2]{sku,qty,price}:\nA1,2,9.99\nB2,1,14.50\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_table_with_nulls() {
    let input = "  id  |  title  |  body  \n------+---------+--------\n   1  | Note A  |        \n   2  |         | Simple note\n";
    let expected = "result[2]{id,title,body}:\n1,Note A,\n2,,Simple note\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_empty_result() {
    let input = "  id  |  name  \n------+--------\n(0 rows)\n";
    let expected = "result[0]{id,name}:\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_single_row() {
    let input = "  id  |  name  \n------+--------\n   1  | Alice  \n(1 row)\n";
    let expected = "result[1]{id,name}:\n1,Alice\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_single_column() {
    let input = "  id  \n------\n   1  \n   2  \n   3  \n";
    let expected = "result[3]{id}:\n1\n2\n3\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_multiple_columns() {
    let input = "  a  |  b  |  c  |  d  |  e  \n-----+-----+-----+-----+-----\n  1  |  2  |  3  |  4  |  5  \n";
    let expected = "result[1]{a,b,c,d,e}:\n1,2,3,4,5\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_table_without_footer() {
    // Some psql outputs don't include the (N rows) footer
    let input = "  id  |  name  \n------+--------\n   1  | Alice  \n   2  | Bob    \n";
    let expected = "result[2]{id,name}:\n1,Alice\n2,Bob\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_data_with_commas_gets_quoted() {
    let input = "  id  |  description      \n------+-------------------\n   1  | Hello, World     \n   2  | Foo, Bar, Baz    \n";
    let expected = "result[2]{id,description}:\n1,\"Hello, World\"\n2,\"Foo, Bar, Baz\"\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_data_with_quotes_gets_escaped() {
    let input = "  id  |  text            \n------+------------------\n   1  | Quote \"test\"    \n";
    let expected = "result[1]{id,text}:\n1,\"Quote \"\"test\"\"\"\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_unicode_data() {
    let input = "  id  |  name   \n------+---------\n   1  | José    \n   2  | 北京     \n";
    let expected = "result[2]{id,name}:\n1,José\n2,北京\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_column_names_with_underscores() {
    let input = "  user_id  |  first_name  |  last_name  \n-----------+--------------+-------------\n     1     |    John      |    Doe      \n";
    let expected = "result[1]{user_id,first_name,last_name}:\n1,John,Doe\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_mixed_data_types() {
    let input = "  id  |  price  |  active  |  created_at         \n------+---------+----------+---------------------\n   1  |  9.99   |  t       | 2025-01-01 10:00:00 \n   2  | 14.50   |  f       | 2025-01-02 11:30:00 \n";
    let expected = "result[2]{id,price,active,created_at}:\n1,9.99,t,2025-01-01 10:00:00\n2,14.50,f,2025-01-02 11:30:00\n";

    run_converter(input)
        .success()
        .stdout(predicate::eq(expected));
}

#[test]
fn test_large_table() {
    // Test with a larger number of rows
    let mut input = "  id  |  value  \n------+---------\n".to_string();
    for i in 1..=100 {
        input.push_str(&format!("   {}  | data{}   \n", i, i));
    }

    run_converter(&input)
        .success()
        .stdout(predicate::str::starts_with("result[100]{id,value}:\n"));
}
