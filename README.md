# TOSE - Token-Optimized SQL Exchange

**Experimental** pipe tools for passing SQL query results to terminal-based coding agents.

## What is this?

This is an early-stage experiment in creating simple pipe tools for terminal workflows with coding agents (like Claude). The goal is to explore whether different output formats help with token usage or information clarity when piping database query results into LLM contexts.

**⚠️ Very untested concept at this stage** - just playing around with ideas for how to structure data for LLM consumption via terminal pipes.

TOSE attempts to combine CSV compactness with structural explicitness through a schema header.

### Format Example

**Input** (PostgreSQL default output):
```
  id  |  name   |  email
------+---------+------------------
   1  | Alice   | alice@example.com
   2  | Bob     | bob@example.com
(2 rows)
```

**Output** (TOSE format):
```
result[2]{id,name,email}:
1,Alice,alice@example.com
2,Bob,bob@example.com
```

The format consists of:
- **Schema header**: `result[2]{id,name,email}:` - describes the data structure
- **CSV data block**: Standard RFC 4180 CSV with proper escaping

## Installation

### Build from Source

```bash
cd tose_converter
cargo build --release
```

The binary will be at `tose_converter/target/release/tose_converter`.

## Usage

### Zero-Friction Workflow

The tool requires **no arguments** - just pipe psql output:

```bash
# Basic query
psql -c "SELECT * FROM users" | tose_converter

# With database connection
psql -d my_db -c "SELECT id, name, email FROM users LIMIT 10" | tose_converter

# Filter results
psql -c "SELECT * FROM users WHERE active = true" | tose_converter
```

### Output Example

```bash
$ psql -c "SELECT id, username, created_at FROM users LIMIT 3" | tose_converter
result[3]{id,username,created_at}:
1,alice,2025-01-01 10:00:00
2,bob,2025-01-02 11:30:00
3,charlie,2025-01-03 09:15:00
```

### Piping to LLMs

```bash
# To file for LLM input
psql -c "SELECT * FROM orders WHERE status = 'pending'" | tose_converter > orders.tose

# Direct to API (example with curl)
psql -c "SELECT * FROM errors LIMIT 100" | \
  tose_converter | \
  jq -Rs '{model: "claude-3-5-sonnet", prompt: "Analyze these errors:\n\(.)"}' | \
  curl -X POST https://api.anthropic.com/v1/messages ...
```

## Features

### ✨ Zero-Friction UX
- **No CLI arguments needed** - column names auto-extracted from psql output
- **No format conversion** - works with psql's default table format
- **Just pipe** - `psql -c "..." | tose_converter`

### 🚀 Optimized for LLMs
- **Compact** - CSV-like data block minimizes tokens
- **Self-documenting** - header includes column names and row count
- **Structured** - easier for LLMs to parse than plain CSV

### 💪 Robust
- **NULL handling** - empty cells properly handled
- **CSV escaping** - commas, quotes, newlines properly escaped
- **Error messages** - clear feedback for invalid input
- **High performance** - streaming I/O, handles gigabytes of data

## Edge Cases Handled

### Empty Results
```bash
$ psql -c "SELECT * FROM users WHERE false" | tose_converter
result[0]{id,name,email}:
```

### NULL Values
```
Input:   1  | Alice   |
Output:  1,Alice,
```

### Special Characters
```
Input:   1  | Hello, World    | Say "Hi"
Output:  1,"Hello, World","Say ""Hi"""
```

## Potential Token Savings (Untested)

The hypothesis is that TOSE might save tokens compared to JSON for tabular data, but **this hasn't been rigorously tested yet**.

**JSON** (verbose):
```json
[
  {"id": 1, "name": "Alice", "email": "alice@example.com"},
  {"id": 2, "name": "Bob", "email": "bob@example.com"}
]
```

**TOSE** (compact):
```
result[2]{id,name,email}:
1,Alice,alice@example.com
2,Bob,bob@example.com
```

Whether this actually helps with LLM understanding or token efficiency is still TBD.

## Technical Details

- **Language**: Rust (edition 2024)
- **Input**: PostgreSQL ASCII table format (default psql output)
- **Output**: TOSE format (schema header + RFC 4180 CSV)
- **Dependencies**: None (core), tempfile (tests)
- **Performance**: Streaming I/O, buffered writes

## Development

```bash
# Run tests
cd tose_converter
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Build release
cargo build --release
```

## Design Principles

1. **Zero friction** - no configuration, no arguments, just works
2. **Token efficiency** - minimize LLM token consumption
3. **Robustness** - handle all edge cases gracefully
4. **Performance** - stream large datasets efficiently
