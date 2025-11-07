use std::io::{self, BufRead, BufReader, Read, Write};

/// Represents a parsed psql table
#[derive(Debug)]
struct PsqlTable {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl PsqlTable {
    /// Parse a psql ASCII table from input
    fn parse<R: Read>(input: R) -> io::Result<Self> {
        let reader = BufReader::new(input);
        let lines: Vec<String> = reader.lines().collect::<io::Result<Vec<_>>>()?;

        if lines.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Empty input: no data to parse",
            ));
        }

        // Find the first separator line (contains --- and +)
        let separator_idx = lines
            .iter()
            .position(|line| Self::is_separator_line(line))
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Input does not appear to be a psql table (no separator line found)",
                )
            })?;

        if separator_idx == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Malformed table: separator found at first line (missing header)",
            ));
        }

        // The line before the separator is the header
        let header_line = &lines[separator_idx - 1];
        let columns = Self::parse_header(header_line)?;

        // Parse data rows (between separator and footer)
        let mut rows = Vec::new();
        for line in lines.iter().skip(separator_idx + 1) {
            // Stop at footer (e.g., "(3 rows)") or another separator
            if Self::is_footer_line(line) || Self::is_separator_line(line) {
                break;
            }

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            let row = Self::parse_row(line, columns.len())?;
            rows.push(row);
        }

        Ok(PsqlTable { columns, rows })
    }

    /// Check if a line is a separator (contains ---, optionally with +)
    fn is_separator_line(line: &str) -> bool {
        // Separator lines have multiple dashes, and may have + for multi-column tables
        line.contains("---")
    }

    /// Check if a line is a footer (e.g., "(3 rows)")
    fn is_footer_line(line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.starts_with('(') && trimmed.ends_with(')') && trimmed.contains("row")
    }

    /// Parse the header line to extract column names
    fn parse_header(line: &str) -> io::Result<Vec<String>> {
        let columns: Vec<String> = line
            .split('|')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if columns.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No columns found in header",
            ));
        }

        Ok(columns)
    }

    /// Parse a data row
    fn parse_row(line: &str, expected_cols: usize) -> io::Result<Vec<String>> {
        // Split by pipe and trim each cell
        // psql format: "  val1  |  val2  |  val3  " (no leading/trailing pipes)
        // Empty cells after trimming represent NULL values
        let result: Vec<String> = line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();

        if result.len() != expected_cols {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Column count mismatch: expected {}, found {}",
                    expected_cols,
                    result.len()
                ),
            ));
        }

        Ok(result)
    }

    /// Write the table as CSV
    fn write_csv<W: Write>(&self, mut output: W) -> io::Result<()> {
        for row in &self.rows {
            let csv_row = row
                .iter()
                .map(|cell| Self::escape_csv_field(cell))
                .collect::<Vec<_>>()
                .join(",");
            writeln!(output, "{}", csv_row)?;
        }
        Ok(())
    }

    /// Escape a field for CSV output (RFC 4180)
    fn escape_csv_field(field: &str) -> String {
        // Empty string means NULL
        if field.is_empty() {
            return String::new();
        }

        // If field contains comma, quote, or newline, it needs to be quoted
        if field.contains(',') || field.contains('"') || field.contains('\n') || field.contains('\r') {
            // Escape quotes by doubling them
            let escaped = field.replace('"', "\"\"");
            format!("\"{}\"", escaped)
        } else {
            field.to_string()
        }
    }

    /// Get row count
    fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Core TOSE converter that transforms psql table data into TOSE format
pub struct ToseConverter {
    entity_name: String,
}

impl ToseConverter {
    /// Create a new TOSE converter with default entity name "result"
    pub fn new() -> Self {
        ToseConverter {
            entity_name: "result".to_string(),
        }
    }

    /// Convert psql table data from input stream to TOSE format on output stream
    pub fn convert<R: Read, W: Write>(&self, input: R, mut output: W) -> io::Result<()> {
        // Parse the psql table
        let table = PsqlTable::parse(input)?;

        // Generate and write the TOSE header
        let header = self.generate_header(table.row_count(), &table.columns);
        output.write_all(header.as_bytes())?;

        // Write the table as CSV
        table.write_csv(&mut output)?;

        Ok(())
    }

    /// Generate the TOSE schema header
    fn generate_header(&self, row_count: usize, columns: &[String]) -> String {
        let field_list = columns.join(",");
        format!(
            "{}[{}]{{{}}}:\n",
            self.entity_name, row_count, field_list
        )
    }
}

impl Default for ToseConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_table() {
        let input = b"  id  |  name  \n------+--------\n   1  | Alice  \n   2  | Bob    \n(2 rows)\n";
        let table = PsqlTable::parse(&input[..]).unwrap();
        assert_eq!(table.columns, vec!["id", "name"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["1", "Alice"]);
        assert_eq!(table.rows[1], vec!["2", "Bob"]);
    }

    #[test]
    fn test_parse_table_without_footer() {
        let input = b"  id  |  name  \n------+--------\n   1  | Alice  \n   2  | Bob    \n";
        let table = PsqlTable::parse(&input[..]).unwrap();
        assert_eq!(table.columns, vec!["id", "name"]);
        assert_eq!(table.rows.len(), 2);
    }

    #[test]
    fn test_parse_table_with_nulls() {
        let input = b"  id  |  name  | email\n------+--------+-------\n   1  | Alice  | \n   2  |        | bob@example.com\n";
        let table = PsqlTable::parse(&input[..]).unwrap();
        assert_eq!(table.rows[0][2], ""); // NULL email
        assert_eq!(table.rows[1][1], ""); // NULL name
    }

    #[test]
    fn test_parse_single_column() {
        let input = b"  id  \n------\n   1  \n   2  \n";
        let table = PsqlTable::parse(&input[..]).unwrap();
        assert_eq!(table.columns, vec!["id"]);
        assert_eq!(table.rows.len(), 2);
    }

    #[test]
    fn test_parse_empty_result() {
        let input = b"  id  |  name  \n------+--------\n(0 rows)\n";
        let table = PsqlTable::parse(&input[..]).unwrap();
        assert_eq!(table.columns, vec!["id", "name"]);
        assert_eq!(table.rows.len(), 0);
    }

    #[test]
    fn test_parse_error_no_separator() {
        let input = b"id,name\n1,Alice\n";
        let result = PsqlTable::parse(&input[..]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no separator line found"));
    }

    #[test]
    fn test_parse_error_empty_input() {
        let input = b"";
        let result = PsqlTable::parse(&input[..]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Empty input"));
    }

    #[test]
    fn test_csv_escaping_commas() {
        let field = "Hello, World";
        let escaped = PsqlTable::escape_csv_field(field);
        assert_eq!(escaped, "\"Hello, World\"");
    }

    #[test]
    fn test_csv_escaping_quotes() {
        let field = "Quote \"test\"";
        let escaped = PsqlTable::escape_csv_field(field);
        assert_eq!(escaped, "\"Quote \"\"test\"\"\"");
    }

    #[test]
    fn test_csv_escaping_null() {
        let field = "";
        let escaped = PsqlTable::escape_csv_field(field);
        assert_eq!(escaped, "");
    }

    #[test]
    fn test_convert_simple_table() {
        let converter = ToseConverter::new();
        let input = b"  id  |  name  \n------+--------\n   1  | Alice  \n   2  | Bob    \n(2 rows)\n";
        let mut output = Vec::new();

        converter.convert(&input[..], &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "result[2]{id,name}:\n1,Alice\n2,Bob\n");
    }

    #[test]
    fn test_convert_with_commas_in_data() {
        let converter = ToseConverter::new();
        let input = b"  id  |  description  \n------+---------------\n   1  | Hello, World  \n";
        let mut output = Vec::new();

        converter.convert(&input[..], &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "result[1]{id,description}:\n1,\"Hello, World\"\n");
    }

    #[test]
    fn test_convert_empty_result() {
        let converter = ToseConverter::new();
        let input = b"  id  |  name  \n------+--------\n(0 rows)\n";
        let mut output = Vec::new();

        converter.convert(&input[..], &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "result[0]{id,name}:\n");
    }

    #[test]
    fn test_generate_header() {
        let converter = ToseConverter::new();
        let columns = vec!["id".to_string(), "name".to_string(), "email".to_string()];
        let header = converter.generate_header(42, &columns);
        assert_eq!(header, "result[42]{id,name,email}:\n");
    }
}
