use tose_converter::ToseConverter;

#[test]
fn test_large_dataset_10k_rows() {
    let converter = ToseConverter::new();

    // Generate a psql table with 10,000 rows
    let mut input_data = "  id  |  value      \n------+-------------\n".to_string();
    for i in 0..10_000 {
        input_data.push_str(&format!("   {}  | value_{}    \n", i, i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[10000]{id,value}:"));

    // Verify first and last rows are present
    assert!(result.contains("0,value_0\n"));
    assert!(result.contains("9999,value_9999\n"));
}

#[test]
fn test_very_large_dataset_100k_rows() {
    let converter = ToseConverter::new();

    // Generate a psql table with 100,000 rows
    let mut input_data = "  n  \n-----\n".to_string();
    for i in 0..100_000 {
        input_data.push_str(&format!("  {}  \n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[100000]{n}:"));
}

#[test]
fn test_long_values() {
    // Test handling of very long individual values
    let converter = ToseConverter::new();

    let long_value = "x".repeat(50_000);
    let input = format!("  id  |  data       \n------+-------------\n   1  | {}  \n   2  | short       \n", long_value);

    let mut output = Vec::new();
    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[2]{id,data}:"));
    assert!(result.contains(&long_value));
    assert!(result.contains("2,short"));
}

#[test]
fn test_many_columns() {
    // Test handling of rows with many columns (100 columns)
    let converter = ToseConverter::new();

    let mut header = String::new();
    let mut separator = String::new();
    let mut values_row = String::new();

    for i in 0..100 {
        if i > 0 {
            header.push_str(" | ");
            separator.push('+');
            values_row.push_str(" | ");
        }
        header.push_str(&format!("col_{}", i));
        separator.push_str("-------");
        values_row.push_str(&format!("    {}  ", i));
    }

    let input = format!("{}\n{}\n{}\n", header, separator, values_row);

    let mut output = Vec::new();
    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[1]{col_0,col_1,"));
    assert!(result.contains(",col_99}:"));
}

#[test]
fn test_mixed_value_sizes() {
    // Test dataset with varying value lengths
    let converter = ToseConverter::new();

    let mut input_data = "  id  |  data       \n------+-------------\n".to_string();

    // Add 1000 rows with varying lengths
    for i in 0..1000 {
        let data = "x".repeat((i % 100) + 1);
        input_data.push_str(&format!("   {}  | {}  \n", i, data));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[1000]{id,data}:"));
}

#[test]
fn test_values_requiring_csv_quoting() {
    // Test performance with many values that need CSV quoting
    let converter = ToseConverter::new();

    let mut input_data = "  id  |  text                     \n------+---------------------------\n".to_string();
    for i in 0..5000 {
        input_data.push_str(&format!("   {}  | Text with, comma \"quote\" \n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[5000]{id,text}:"));
    // Verify CSV escaping happened
    assert!(result.contains("\"Text with, comma \"\"quote\"\"\""));
}

#[test]
fn test_many_null_values() {
    // Test with many NULL values
    let converter = ToseConverter::new();

    let mut input_data = "  a  |  b  |  c  \n-----+-----+-----\n".to_string();
    for i in 0..10_000 {
        if i % 3 == 0 {
            input_data.push_str(&format!("   {}  |     |     \n", i));
        } else if i % 3 == 1 {
            input_data.push_str(&format!("      |  {}  |     \n", i));
        } else {
            input_data.push_str(&format!("      |     |  {}  \n", i));
        }
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[10000]{a,b,c}:"));
}

#[test]
fn test_minimal_memory_footprint() {
    // Verify that we can process data larger than what we'd want in memory at once
    let converter = ToseConverter::new();

    // Generate 50,000 rows (approximately 500KB of data)
    let mut input_data = "  id  \n------\n".to_string();
    for i in 0..50_000 {
        input_data.push_str(&format!("   {}  \n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[50000]{id}:"));

    // Verify sample rows
    assert!(result.contains("0\n"));
    assert!(result.contains("25000\n"));
    assert!(result.contains("49999\n"));
}

#[test]
fn test_unicode_heavy_dataset() {
    // Test performance with Unicode characters
    let converter = ToseConverter::new();

    let mut input_data = "  id  |  text               \n------+---------------------\n".to_string();
    for i in 0..5000 {
        input_data.push_str(&format!("   {}  | 你好世界 🚀 émojis  \n", i));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[5000]{id,text}:"));
    assert!(result.contains("你好世界 🚀 émojis"));
}

#[test]
fn test_numeric_precision() {
    // Test with high-precision numeric values
    let converter = ToseConverter::new();

    let mut input_data = "  id  |  price         \n------+----------------\n".to_string();
    for i in 0..1000 {
        input_data.push_str(&format!("   {}  | {}.{:06}      \n", i, i, i * 123456 % 1000000));
    }

    let mut output = Vec::new();
    converter.convert(input_data.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[1000]{id,price}:"));
}

#[test]
fn test_wide_and_deep_table() {
    // Test a table that's both wide (many columns) and deep (many rows)
    let converter = ToseConverter::new();

    // 20 columns × 5000 rows
    let num_cols = 20;
    let num_rows = 5000;

    let mut input = String::new();

    // Build header
    for i in 0..num_cols {
        if i > 0 {
            input.push_str(" | ");
        }
        input.push_str(&format!("c{}", i));
    }
    input.push('\n');

    // Build separator
    for i in 0..num_cols {
        if i > 0 {
            input.push('+');
        }
        input.push_str("----");
    }
    input.push('\n');

    // Build data rows
    for row in 0..num_rows {
        for col in 0..num_cols {
            if col > 0 {
                input.push_str(" | ");
            }
            input.push_str(&format!(" {}", row * num_cols + col));
        }
        input.push('\n');
    }

    let mut output = Vec::new();
    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[5000]{c0,c1,"));
}
