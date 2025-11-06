use tose_converter::ToseConverter;

#[test]
fn test_count_empty_input() {
    let converter = ToseConverter::new("test".to_string(), vec![]);
    let input = b"";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[0]{}:\n");
}

#[test]
fn test_count_single_row_no_newline() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"data";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[1]{col}:\ndata");
}

#[test]
fn test_count_single_row_with_newline() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"data\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[1]{col}:\ndata\n");
}

#[test]
fn test_count_multiple_rows() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"row1\nrow2\nrow3\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "test[3]{col}:\nrow1\nrow2\nrow3\n");
}

#[test]
fn test_count_empty_lines_not_counted() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"row1\n\nrow2\n\n\nrow3\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // Should count only 3 non-empty rows
    assert!(result.starts_with("test[3]{col}:"));
    // But preserve all lines in output
    assert_eq!(result, "test[3]{col}:\nrow1\n\nrow2\n\n\nrow3\n");
}

#[test]
fn test_count_whitespace_only_lines_not_counted() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"row1\n   \nrow2\n\t\nrow3\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // Should count only 3 rows (whitespace-only lines excluded)
    assert!(result.starts_with("test[3]{col}:"));
    // But preserve all lines in output
    assert_eq!(result, "test[3]{col}:\nrow1\n   \nrow2\n\t\nrow3\n");
}

#[test]
fn test_count_mixed_whitespace() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"row1\n \t \nrow2\n\n\t\nrow3\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[3]{col}:"));
}

#[test]
fn test_count_only_newline() {
    let converter = ToseConverter::new("test".to_string(), vec![]);
    let input = b"\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // Single newline is an empty line, should not be counted
    assert_eq!(result, "test[0]{}:\n\n");
}

#[test]
fn test_count_multiple_newlines_only() {
    let converter = ToseConverter::new("test".to_string(), vec![]);
    let input = b"\n\n\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // All empty lines, count should be 0
    assert_eq!(result, "test[0]{}:\n\n\n\n");
}

#[test]
fn test_count_large_number_of_rows() {
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string()]);
    // Generate 1000 rows
    let mut input_data = String::new();
    for i in 0..1000 {
        input_data.push_str(&format!("{}\n", i));
    }
    let mut output = Vec::new();

    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[1000]{id}:"));
}

#[test]
fn test_count_rows_with_varying_lengths() {
    let converter = ToseConverter::new("test".to_string(), vec!["data".to_string()]);
    let input = b"a\nabcdefghijklmnop\nxy\nverylongdataverylongdataverylongdata\nz\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[5]{data}:"));
}

#[test]
fn test_count_csv_rows_with_commas() {
    let converter = ToseConverter::new("test".to_string(), vec!["a".to_string(), "b".to_string()]);
    let input = b"1,2\n3,4\n5,6\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[3]{a,b}:"));
}

#[test]
fn test_count_rows_ending_without_newline() {
    let converter = ToseConverter::new("test".to_string(), vec!["col".to_string()]);
    let input = b"row1\nrow2\nrow3";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // All three rows should be counted
    assert!(result.starts_with("test[3]{col}:"));
}
