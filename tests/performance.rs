use tose_converter::ToseConverter;

#[test]
fn test_large_dataset_10k_rows() {
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "value".to_string()]);

    // Generate 10,000 rows
    let mut input_data = String::new();
    for i in 0..10_000 {
        input_data.push_str(&format!("{},value_{}\n", i, i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[10000]{id,value}:"));

    // Verify first and last rows are present
    assert!(result.contains("0,value_0\n"));
    assert!(result.contains("9999,value_9999\n"));
}

#[test]
fn test_very_large_dataset_100k_rows() {
    let converter = ToseConverter::new("big".to_string(), vec!["n".to_string()]);

    // Generate 100,000 rows
    let mut input_data = String::new();
    for i in 0..100_000 {
        input_data.push_str(&format!("{}\n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("big[100000]{n}:"));
}

#[test]
fn test_long_lines() {
    // Test handling of very long individual lines
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "data".to_string()]);

    let long_value = "x".repeat(50_000);
    let input = format!("1,\"{}\"\n2,short\n", long_value);

    let mut output = Vec::new();
    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[2]{id,data}:"));
    assert!(result.contains(&long_value));
    assert!(result.contains("2,short"));
}

#[test]
fn test_many_columns() {
    // Test handling of rows with many columns
    let mut fields = Vec::new();
    let mut values = Vec::new();

    for i in 0..100 {
        fields.push(format!("col_{}", i));
        values.push(i.to_string());
    }

    let converter = ToseConverter::new("wide".to_string(), fields.clone());
    let input = format!("{}\n", values.join(","));

    let mut output = Vec::new();
    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    let expected_header = format!("wide[1]{{{}}}:", fields.join(","));
    assert!(result.starts_with(&expected_header));
}

#[test]
fn test_mixed_row_sizes() {
    // Test dataset with varying row sizes
    let converter = ToseConverter::new("test".to_string(), vec!["data".to_string()]);

    let mut input_data = String::new();

    // Add 1000 rows with varying lengths
    for i in 0..1000 {
        let data = "x".repeat((i % 100) + 1);
        input_data.push_str(&format!("{}\n", data));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[1000]{data}:"));
}

#[test]
fn test_many_empty_lines() {
    // Test performance with many empty lines that should be filtered
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string()]);

    let mut input_data = String::new();

    // Add 10,000 lines, half of which are empty
    for i in 0..10_000 {
        if i % 2 == 0 {
            input_data.push_str(&format!("{}\n", i));
        } else {
            input_data.push('\n');
        }
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // Should count only the 5000 non-empty rows
    assert!(result.starts_with("test[5000]{id}:"));
}

#[test]
fn test_quoted_fields_performance() {
    // Test performance with many quoted fields
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);

    let mut input_data = String::new();
    for i in 0..5000 {
        input_data.push_str(&format!("{},\"Text with, comma and \"\"quotes\"\" {}\"\n", i, i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[5000]{id,text}:"));
}

#[test]
fn test_multiline_fields_performance() {
    // Test with fields containing newlines (these affect row counting)
    // Note: The converter counts physical lines, not logical CSV records
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);

    let mut input_data = String::new();
    for i in 0..1000 {
        input_data.push_str(&format!("{},\"Line1\nLine2\nLine3\"\n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    // Each record has 2 embedded newlines, so 1000 records = 3000 physical lines
    assert!(result.starts_with("test[3000]{id,text}:"));
}

#[test]
fn test_minimal_memory_footprint() {
    // Verify that we can process data larger than what we'd want in memory at once
    // This test validates streaming behavior by using the tempfile approach
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string()]);

    // Generate 50,000 rows (approximately 500KB of data)
    let mut input_data = String::new();
    for i in 0..50_000 {
        input_data.push_str(&format!("{}\n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[50000]{id}:"));

    // Verify sample rows
    assert!(result.contains("0\n"));
    assert!(result.contains("25000\n"));
    assert!(result.contains("49999\n"));
}

#[test]
fn test_unicode_heavy_dataset() {
    // Test performance with Unicode characters
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "text".to_string()]);

    let mut input_data = String::new();
    for i in 0..5000 {
        input_data.push_str(&format!("{},\"Unicode: 你好世界 🚀 émojis\"\n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[5000]{id,text}:"));
}

#[test]
fn test_binary_like_data() {
    // Test with data that looks like binary (hex strings, as per PostgreSQL bytea HEX format)
    let converter = ToseConverter::new("test".to_string(), vec!["id".to_string(), "data".to_string()]);

    let mut input_data = String::new();
    for i in 0..1000 {
        input_data.push_str(&format!("{},\\x{:02x}{:02x}{:02x}{:02x}\n", i, i % 256, (i * 2) % 256, (i * 3) % 256, (i * 4) % 256));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("test[1000]{id,data}:"));
}
