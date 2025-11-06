use tose_converter::ToseConverter;

/// Test that RFC 4180 CSV data passes through the converter unchanged

#[test]
fn test_simple_unquoted_values() {
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let input = b"1,2,3\n4,5,6\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[2]{a,b,c}:\n1,2,3\n4,5,6\n");
}

#[test]
fn test_null_values_empty_fields() {
    // RFC 4180: NULL values are represented as empty fields
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let input = b"1,,3\n,5,\n,,\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[3]{a,b,c}:\n1,,3\n,5,\n,,\n");
}

#[test]
fn test_quoted_field_with_comma() {
    // RFC 4180: Fields containing commas must be quoted
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "desc".to_string()]);
    let input = b"1,\"Hello, World\"\n2,\"Item, Blue\"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[2]{id,desc}:\n1,\"Hello, World\"\n2,\"Item, Blue\"\n");
}

#[test]
fn test_quoted_field_with_double_quotes() {
    // RFC 4180: Fields containing double quotes must be quoted, and quotes doubled
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);
    let input = b"1,\"Part \"\"X\"\"\"\n2,\"Say \"\"Hello\"\"\"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[2]{id,text}:\n1,\"Part \"\"X\"\"\"\n2,\"Say \"\"Hello\"\"\"\n");
}

#[test]
fn test_quoted_field_with_newline() {
    // RFC 4180: Fields containing newlines must be quoted
    // Note: The converter counts physical lines (by \n), not logical CSV records
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);
    let input = b"1,\"Line1\nLine2\"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // This counts as 2 physical lines because of the embedded newline
    assert_eq!(result, "test[2]{id,text}:\n1,\"Line1\nLine2\"\n");
}

#[test]
fn test_quoted_field_with_comma_and_quotes() {
    // RFC 4180: Combined escaping - both commas and quotes
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);
    let input = b"1,\"Item \"\"A\"\", Qty: 2\"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[1]{id,text}:\n1,\"Item \"\"A\"\", Qty: 2\"\n");
}

#[test]
fn test_consecutive_quoted_fields() {
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let input = b"\"Hello, A\",\"Hello, B\",\"Hello, C\"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[1]{a,b,c}:\n\"Hello, A\",\"Hello, B\",\"Hello, C\"\n");
}

#[test]
fn test_mixed_quoted_and_unquoted() {
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "name".to_string(), "desc".to_string()]);
    let input = b"1,Alice,\"Developer, Senior\"\n2,\"Bob, Jr.\",Engineer\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[2]{id,name,desc}:\n1,Alice,\"Developer, Senior\"\n2,\"Bob, Jr.\",Engineer\n");
}

#[test]
fn test_quoted_empty_string() {
    // Quoted empty string is different from NULL (empty field)
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let input = b"1,\"\",3\n4,,6\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[2]{a,b,c}:\n1,\"\",3\n4,,6\n");
}

#[test]
fn test_leading_and_trailing_spaces_in_quoted_field() {
    // RFC 4180: Spaces inside quotes are preserved
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string()]);
    let input = b"1,\"  spaces  \"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[1]{a,b}:\n1,\"  spaces  \"\n");
}

#[test]
fn test_all_special_characters_combined() {
    // Kitchen sink test: commas, quotes, newlines, empty fields
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "data".to_string()]);
    let input = b"1,\"Line1\nLine2, with comma\nand \"\"quotes\"\"\"\n2,\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // The first field has 2 embedded newlines, so total is 4 physical lines (3 from first record + 1 from second)
    assert_eq!(result, "test[4]{id,data}:\n1,\"Line1\nLine2, with comma\nand \"\"quotes\"\"\"\n2,\n");
}

#[test]
fn test_spec_example_complex_notes() {
    // From TOSE spec example 7.2
    let converter = ToseConverter::new("notes".to_string(), vec!["id".to_string(), "title".to_string(), "body".to_string()]);
    let input = b"1,\"Note \"\"A\"\"\",\n2,\"\"\"Comma, Inc.\"\"\",A simple note.\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "notes[2]{id,title,body}:\n1,\"Note \"\"A\"\"\",\n2,\"\"\"Comma, Inc.\"\"\",A simple note.\n");
}

#[test]
fn test_unicode_characters_preserved() {
    // Ensure non-ASCII Unicode characters pass through unchanged
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);
    let input = "1,\"Hello 世界\"\n2,\"Emoji 🚀\"\n".as_bytes();
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[2]{id,text}:\n1,\"Hello 世界\"\n2,\"Emoji 🚀\"\n");
}

#[test]
fn test_carriage_return_in_data() {
    // Some systems use CRLF line endings
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string()]);
    let input = b"1,2\r\n3,4\r\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // Should preserve CRLF line endings
    assert_eq!(result, "test[2]{a,b}:\n1,2\r\n3,4\r\n");
}

#[test]
fn test_tab_characters_in_unquoted_field() {
    // Tabs in unquoted fields should be preserved
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string()]);
    let input = b"1\t2,data\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[1]{a,b}:\n1\t2,data\n");
}

#[test]
fn test_very_long_quoted_field() {
    // Test handling of fields with thousands of characters
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);
    let long_text = "x".repeat(10000);
    let input = format!("1,\"{}\"", long_text);
    let mut output = Vec::new();

    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[1]{id,text}:\n1,\""));
    assert!(result.contains(&long_text));
}
