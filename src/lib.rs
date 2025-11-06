use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use tempfile::NamedTempFile;

/// Core TOSE converter that transforms CSV data into TOSE format
pub struct ToseConverter {
    entity_name: String,
    field_list: String,
}

impl ToseConverter {
    /// Create a new TOSE converter with the given entity name and field names
    pub fn new(entity_name: String, fields: Vec<String>) -> Self {
        let field_list = fields.join(",");
        ToseConverter {
            entity_name,
            field_list,
        }
    }

    /// Convert CSV data from input stream to TOSE format on output stream
    pub fn convert<R: Read, W: Write>(&self, input: R, mut output: W) -> io::Result<()> {
        // Count rows and buffer data using a temporary file
        let (row_count, temp_file) = Self::count_rows_and_buffer(input)?;

        // Generate and write the TOSE header
        let header = self.generate_header(row_count);
        output.write_all(header.as_bytes())?;

        // Stream the buffered data to output
        let mut temp_reader = BufReader::new(temp_file);
        io::copy(&mut temp_reader, &mut output)?;

        Ok(())
    }

    /// Count non-empty rows in the input and buffer data to a temporary file
    fn count_rows_and_buffer<R: Read>(input: R) -> io::Result<(usize, File)> {
        let mut reader = BufReader::new(input);
        let mut temp_file = NamedTempFile::new()?;
        let mut row_count = 0usize;
        let mut buffer = Vec::new();

        loop {
            buffer.clear();
            let bytes_read = reader.read_until(b'\n', &mut buffer)?;

            if bytes_read == 0 {
                break; // EOF
            }

            // Write the line to the temp file
            temp_file.write_all(&buffer)?;

            // Count non-empty, non-whitespace-only lines
            if !buffer.iter().all(|&b| b.is_ascii_whitespace()) {
                row_count += 1;
            }
        }

        // Rewind the temp file to the beginning for reading
        temp_file.flush()?;
        let temp_file = temp_file.reopen()?;

        Ok((row_count, temp_file))
    }

    /// Generate the TOSE schema header
    fn generate_header(&self, row_count: usize) -> String {
        format!(
            "{}[{}]{{{}}}:\n",
            self.entity_name, row_count, self.field_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_header_with_fields() {
        let converter = ToseConverter::new("users".to_string(), vec!["id".to_string(), "name".to_string()]);
        let header = converter.generate_header(42);
        assert_eq!(header, "users[42]{id,name}:\n");
    }

    #[test]
    fn test_generate_header_no_fields() {
        let converter = ToseConverter::new("entity".to_string(), vec![]);
        let header = converter.generate_header(0);
        assert_eq!(header, "entity[0]{}:\n");
    }

    #[test]
    fn test_generate_header_single_field() {
        let converter = ToseConverter::new("items".to_string(), vec!["sku".to_string()]);
        let header = converter.generate_header(10);
        assert_eq!(header, "items[10]{sku}:\n");
    }

    #[test]
    fn test_count_rows_simple() {
        let input = b"row1\nrow2\nrow3\n";
        let (count, _) = ToseConverter::count_rows_and_buffer(&input[..]).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_rows_empty_input() {
        let input = b"";
        let (count, _) = ToseConverter::count_rows_and_buffer(&input[..]).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_count_rows_with_empty_lines() {
        let input = b"row1\n\nrow2\n\n\nrow3\n";
        let (count, _) = ToseConverter::count_rows_and_buffer(&input[..]).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_rows_whitespace_only_lines() {
        let input = b"row1\n   \nrow2\n\t\nrow3\n";
        let (count, _) = ToseConverter::count_rows_and_buffer(&input[..]).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_rows_no_trailing_newline() {
        let input = b"row1";
        let (count, _) = ToseConverter::count_rows_and_buffer(&input[..]).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_convert_simple() {
        let converter = ToseConverter::new("items".to_string(), vec!["sku".to_string(), "qty".to_string()]);
        let input = b"A1,2\nB2,3\n";
        let mut output = Vec::new();

        converter.convert(&input[..], &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "items[2]{sku,qty}:\nA1,2\nB2,3\n");
    }

    #[test]
    fn test_convert_empty() {
        let converter = ToseConverter::new("empty".to_string(), vec![]);
        let input = b"";
        let mut output = Vec::new();

        converter.convert(&input[..], &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "empty[0]{}:\n");
    }

    #[test]
    fn test_convert_preserves_quoted_fields() {
        let converter = ToseConverter::new("notes".to_string(), vec!["id".to_string(), "text".to_string()]);
        let input = b"1,\"Hello, World\"\n2,\"Quote \"\"test\"\"\"\n";
        let mut output = Vec::new();

        converter.convert(&input[..], &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "notes[2]{id,text}:\n1,\"Hello, World\"\n2,\"Quote \"\"test\"\"\"\n");
    }
}
