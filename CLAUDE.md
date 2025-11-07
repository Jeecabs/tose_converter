# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository implements the **Token-Optimized SQL Exchange (TOSE)** format - a compact, human-readable serialization format designed to pipe SQL query results to Large Language Models with significantly reduced token usage compared to JSON. TOSE achieves CSV-like compactness while maintaining structural explicitness through a schema header.

The project consists of `tose_converter`, a high-performance Rust CLI tool that acts as a pipe filter, reading PostgreSQL's default ASCII table format from stdin and converting it to TOSE format on stdout.

## Key Architecture

### TOSE Format Structure

TOSE documents consist of exactly two parts separated by a newline:

1. **Schema Header** (one line): `result[ROW_COUNT]{FIELD_1,FIELD_2,...,FIELD_N}:`
   - Example: `result[150]{id,username,email,last_login,is_active}:`
   - Entity name is always "result"

2. **Data Block**: RFC 4180 CSV-compliant data rows
   - Generated from psql table data with proper escaping

### Pipeline Design

The tool is designed for zero-friction streaming pipelines:
```bash
psql -d my_db -c "SELECT id, username, email FROM users LIMIT 10" | tose_converter
```

Output:
```
result[10]{id,username,email}:
1,alice,alice@example.com
2,bob,bob@example.com
...
```

### Implementation (`tose_converter/src/lib.rs`)

The converter consists of two main components:

#### 1. PsqlTable Parser
Parses PostgreSQL's ASCII table format:
```
  id  |  name   |  email
------+---------+------------------
   1  | Alice   | alice@example.com
   2  | Bob     | bob@example.com
(2 rows)
```

**Parsing Logic:**
1. Find separator line (contains `---`)
2. Extract column names from header (line before separator)
3. Parse data rows (between separator and footer)
4. Handle NULL values (empty cells)
5. Generate RFC 4180 compliant CSV output

#### 2. ToseConverter
1. Parse psql table using PsqlTable::parse()
2. Generate TOSE header with "result" as entity name
3. Write header + CSV data to stdout

**Key Features:**
- **Zero CLI arguments** - everything auto-detected from input
- **Auto-extract column names** from psql table header
- **CSV escaping** - handles commas, quotes, newlines properly
- **NULL handling** - empty cells become empty CSV fields
- **Buffered I/O** for high performance

## Common Commands

### Build
```bash
cd tose_converter
cargo build --release
```

### Run
```bash
# Basic usage - no arguments needed!
psql -c "SELECT * FROM users" | ./target/release/tose_converter

# With database connection
psql -d my_db -c "SELECT id, name, email FROM users WHERE active = true" | tose_converter

# Empty results work too
psql -c "SELECT * FROM users WHERE false" | tose_converter
# Output: result[0]{id,name,email}:
```

### Test
```bash
cd tose_converter
cargo test
```

### Lint
```bash
cd tose_converter
cargo clippy
```

### Format
```bash
cd tose_converter
cargo fmt
```

## Important Implementation Details

### Input Format

The tool expects PostgreSQL's default ASCII table format:
- Header row with column names separated by `|`
- Separator line with `---` (and `+` for multi-column tables)
- Data rows with values separated by `|`
- Optional footer like `(N rows)`

### Data Type Handling

The converter extracts and converts psql output to CSV:
- **NULL**: Empty cells in psql table → empty CSV fields
- **Numeric**: Preserved as-is (e.g., `12345.67`)
- **Text with commas**: Quoted (e.g., `"Hello, World"`)
- **Text with quotes**: Escaped (e.g., `"Say ""Hello"""`)
- **Timestamps**: Preserved as-is
- **Whitespace**: Trimmed from cell boundaries

### Row Counting

Counts data rows between separator line and footer (or EOF). Empty lines and the footer itself are not counted as data rows.

### Column Name Extraction

Column names are extracted from the header row by:
1. Finding the separator line
2. Reading the previous line
3. Splitting on `|` and trimming whitespace
4. Using trimmed names in the TOSE header

### Error Handling

The tool fails with clear error messages for:
- Empty input
- Non-psql format input (no separator line)
- Malformed tables (column count mismatches)

## Development Notes

- The project uses Rust edition 2024
- Dependencies: None for core logic (tempfile for tests)
- Core logic split between `src/lib.rs` (parser + converter) and `src/main.rs` (CLI entry point)
- The tool is designed for high throughput (handles gigabytes of data)
- Error handling returns `io::Result<()>` - uses `?` operator for propagation
- Entity name is hardcoded to "result" for simplicity

## Design Principles

### Zero-Friction UX
- **No CLI arguments** - users don't need to specify column names or entity names
- **No format conversion** - users can use psql's default output, no need for `\copy` or CSV format
- **Just pipe** - `psql -c "..." | tose_converter` is all you need

### Token Efficiency
- Compact schema header (CSV-like data block)
- Self-documenting (includes column names and row count)
- Optimized for LLM token consumption

### Robustness
- Comprehensive error handling
- Extensive test coverage (unit + integration + performance)
- Handles edge cases (NULLs, empty results, special characters)
