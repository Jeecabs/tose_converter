# TOSE Converter

A high-performance Rust CLI tool that converts CSV data to **TOSE (Token-Optimized SQL Exchange)** format - a compact, human-readable serialization format designed to pipe structured SQL query results to Large Language Models with significantly reduced token usage compared to JSON.

## Features

- **Token Efficient**: Achieves CSV-like compactness while maintaining structural explicitness
- **High Performance**: Streaming architecture handles gigabytes of data with minimal memory footprint
- **RFC 4180 Compliant**: Preserves all CSV escaping (quotes, commas, newlines, NULL values)
- **Simple Pipeline Design**: Acts as a Unix-style filter (stdin → stdout)
- **Zero Configuration**: No config files, just command-line arguments

## Installation

### Homebrew (macOS)

```bash
brew tap lachlanjacobs/tose
brew install tose_converter
```

### From Source

Requires [Rust](https://rustup.rs/) 1.88.0 or later:

```bash
git clone https://github.com/lachlanjacobs/tose_converter.git
cd tose_converter
cargo install --path .
```

## Usage

### Basic Syntax

```bash
tose_converter <ENTITY_NAME> [FIELD_1] [FIELD_2] ... [FIELD_N]
```

### Pipeline with PostgreSQL

```bash
psql -d my_db -c "\copy (SELECT sku, qty, price FROM order_items) TO STDOUT WITH (FORMAT CSV, HEADER FALSE)" \
| tose_converter orderItems sku qty price
```

**Output:**
```
orderItems[2]{sku,qty,price}:
A1,2,9.99
B2,1,14.50
```

### Examples

**Simple conversion:**
```bash
echo -e "A1,2,9.99\nB2,1,14.50" | tose_converter orderItems sku qty price
```

**With complex data (quotes, commas, NULL):**
```bash
psql -d my_db -c "\copy (SELECT id, title, body FROM notes) TO STDOUT WITH (FORMAT CSV, HEADER FALSE)" \
| tose_converter notes id title body
```

**Pipe to LLM API:**
```bash
psql -d my_db -c "\copy (SELECT * FROM users LIMIT 100) TO STDOUT WITH (FORMAT CSV, HEADER FALSE)" \
| tose_converter users id username email last_login is_active \
| curl -X POST https://api.anthropic.com/v1/messages \
  -H "Content-Type: application/json" \
  -d @-
```

## TOSE Format

TOSE documents consist of two parts:

1. **Schema Header** (one line): `ENTITY[COUNT]{FIELD1,FIELD2,...}:`
2. **Data Block**: RFC 4180 CSV data

### Example

**Input CSV:**
```csv
1,Alice,alice@example.com
2,Bob,bob@example.com
```

**TOSE Output:**
```
users[2]{id,name,email}:
1,Alice,alice@example.com
2,Bob,bob@example.com
```

### Token Savings

Compared to JSON, TOSE reduces tokens by **60-80%** for tabular data:

**JSON (178 tokens):**
```json
[
  {"id": 1, "name": "Alice", "email": "alice@example.com"},
  {"id": 2, "name": "Bob", "email": "bob@example.com"}
]
```

**TOSE (45 tokens):**
```
users[2]{id,name,email}:
1,Alice,alice@example.com
2,Bob,bob@example.com
```

## Specification

See [TOSE_SPECIFICATION.md](TOSE_SPECIFICATION.md) for the complete format specification, including:
- Escaping rules for special characters
- NULL value handling
- Data type normalization guidelines
- Design principles and rationale

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

The test suite includes 82 comprehensive tests covering:
- TOSE specification examples
- RFC 4180 CSV compliance
- Row counting edge cases
- Performance with large datasets (100K+ rows)
- Unicode and binary data handling

### Lint

```bash
cargo clippy
```

## Architecture

- **Streaming**: Uses temporary files to buffer data while counting rows, enabling constant memory usage regardless of dataset size
- **Zero-copy**: Preserves CSV data byte-for-byte without parsing or re-encoding
- **Performance**: Handles gigabytes of data efficiently with buffered I/O

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a pull request

## Links

- [GitHub Repository](https://github.com/lachlanjacobs/tose_converter)
- [Issue Tracker](https://github.com/lachlanjacobs/tose_converter/issues)
- [TOSE Specification](TOSE_SPECIFICATION.md)
