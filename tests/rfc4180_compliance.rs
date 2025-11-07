use tose_converter::ToseConverter;

/// Test that CSV output generated from psql tables is RFC 4180 compliant

#[test]
fn test_simple_unquoted_values() {
    let converter = ToseConverter::new();
    let input = b"  a  |  b  |  c  \n-----+-----+-----\n  1  |  2  |  3  \n  4  |  5  |  6  \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{a,b,c}:\n1,2,3\n4,5,6\n");
}

#[test]
fn test_null_values_as_empty_fields() {
    // psql shows NULL as empty/whitespace - should become empty CSV fields
    let converter = ToseConverter::new();
    let input = b"  a  |  b  |  c  \n-----+-----+-----\n  1  |     |  3  \n     |  5  |     \n     |     |     \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[3]{a,b,c}:\n1,,3\n,5,\n,,\n");
}

#[test]
fn test_values_with_commas_get_quoted() {
    // RFC 4180: Fields containing commas must be quoted
    let converter = ToseConverter::new();
    let input = b"  id  |  desc           \n------+-----------------\n   1  | Hello, World    \n   2  | Item, Blue      \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{id,desc}:\n1,\"Hello, World\"\n2,\"Item, Blue\"\n");
}

#[test]
fn test_values_with_quotes_get_escaped() {
    // RFC 4180: Fields containing double quotes must be quoted, and quotes doubled
    let converter = ToseConverter::new();
    let input = b"  id  |  text            \n------+------------------\n   1  | Part \"X\"        \n   2  | Say \"Hello\"     \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{id,text}:\n1,\"Part \"\"X\"\"\"\n2,\"Say \"\"Hello\"\"\"\n");
}

#[test]
fn test_values_without_special_chars_unquoted() {
    // Simple values don't need quoting
    let converter = ToseConverter::new();
    let input = b"  id  |  text  \n------+--------\n   1  | Hello  \n   2  | World  \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{id,text}:\n1,Hello\n2,World\n");
}

#[test]
fn test_numeric_values_unquoted() {
    let converter = ToseConverter::new();
    let input = b"  id  |  price  |  qty  \n------+---------+-------\n   1  | 9.99    |  100  \n   2  | 14.50   |  50   \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{id,price,qty}:\n1,9.99,100\n2,14.50,50\n");
}

#[test]
fn test_mixed_quoted_and_unquoted() {
    let converter = ToseConverter::new();
    let input = b"  id  |  simple  |  complex        \n------+----------+-----------------\n   1  | plain    | has, comma      \n   2  | text     | has \"quote\"     \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{id,simple,complex}:\n1,plain,\"has, comma\"\n2,text,\"has \"\"quote\"\"\"\n");
}

#[test]
fn test_empty_string_vs_null() {
    // Both should produce empty CSV field, but psql shows them the same way
    let converter = ToseConverter::new();
    let input = b"  id  |  value  \n------+---------\n   1  |         \n   2  |         \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[2]{id,value}:\n1,\n2,\n");
}
