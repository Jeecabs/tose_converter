use tose_converter::ToseConverter;

#[test]
fn test_count_empty_result() {
    let converter = ToseConverter::new();
    let input = b"  id  |  name  \n------+--------\n(0 rows)\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[0]{id,name}:\n");
}

#[test]
fn test_count_single_row() {
    let converter = ToseConverter::new();
    let input = b"  id  |  name  \n------+--------\n   1  | Alice  \n(1 row)\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[1]{id,name}:\n1,Alice\n");
}

#[test]
fn test_count_multiple_rows() {
    let converter = ToseConverter::new();
    let input = b"  id  |  name  \n------+--------\n   1  | Alice  \n   2  | Bob    \n   3  | Charlie\n(3 rows)\n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[3]{id,name}:\n1,Alice\n2,Bob\n3,Charlie\n");
}

#[test]
fn test_count_without_footer() {
    // Table without (N rows) footer
    let converter = ToseConverter::new();
    let input = b"  id  \n------\n   1  \n   2  \n   3  \n   4  \n   5  \n";
    let mut output = Vec::new();

    converter.convert(&input[..], &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "result[5]{id}:\n1\n2\n3\n4\n5\n");
}

#[test]
fn test_count_large_result() {
    let converter = ToseConverter::new();
    let mut input = "  id  |  value  \n------+---------\n".to_string();
    for i in 1..=1000 {
        input.push_str(&format!("   {}  | data    \n", i));
    }

    let mut output = Vec::new();
    converter.convert(input.as_bytes(), &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.starts_with("result[1000]{id,value}:\n"));
}
