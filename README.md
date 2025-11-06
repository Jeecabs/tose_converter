# TOSE Converter

A Rust CLI tool that adds a schema header to CSV data from PostgreSQL. TOSE (Token-Optimized SQL Exchange) is essentially **CSV with a descriptive header line** that tells an LLM what the data represents.

## What It Does

Converts this:
```csv
1,Alice,alice@example.com
2,Bob,bob@example.com
```

To this:
```
users[2]{id,name,email}:
1,Alice,alice@example.com
2,Bob,bob@example.com
```

The header costs **22 tokens** but provides schema information.

## When To Use This

✅ **Use TOSE when:**
- Sending multiple tables and need clear boundaries
- LLM needs to understand column names without separate explanation
- Debugging/exploring data where schema clarity helps
- 22 tokens is worth it for explainability

❌ **Don't use TOSE when:**
- Sending a single table (plain CSV is 22 tokens cheaper)
- LLM already knows the schema
- You're at absolute token limits
- You can explain the schema elsewhere

## Scientific Benchmarks

Tested with PostgreSQL on 100 rows of real data using tiktoken (GPT-4 tokenizer):

| Format | Tokens | vs JSON | vs CSV |
|--------|--------|---------|---------|
| **JSON** | 5,820 | baseline | +76% |
| **Plain CSV** | 3,308 | **-43%** | baseline |
| **TOSE** | 3,330 | **-43%** | +22 tokens |

**Bottom line:** TOSE and CSV save ~43% vs JSON. TOSE costs 22 extra tokens for the header.

### Multi-Table Scenario

With 3 tables (users, orders, products):

| Format | Tokens | Notes |
|--------|--------|-------|
| **Plain CSV** | 760 | Tables mashed together, no way to tell them apart |
| **TOSE** | 818 | Clear table boundaries, +58 tokens |

TOSE makes more sense here, but honestly you should probably JOIN the tables properly instead.

## Installation

### Homebrew (macOS)

```bash
brew tap jeecabs/tose
brew install tose_converter
```

### From Source

Requires [Rust](https://rustup.rs/) 1.88.0 or later:

```bash
git clone https://github.com/Jeecabs/tose_converter.git
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

## TOSE Format

Two parts:

1. **Schema Header**: `ENTITY[COUNT]{FIELD1,FIELD2,...}:`
2. **Data Block**: Standard RFC 4180 CSV

### Comparison

**Plain CSV (no schema info):**
```csv
1,Alice,alice@example.com
2,Bob,bob@example.com
```
*LLM doesn't know what columns are*

**TOSE (22 tokens for schema):**
```
users[2]{id,name,email}:
1,Alice,alice@example.com
2,Bob,bob@example.com
```
*LLM knows: entity name, row count, column names*

## Honest Assessment

This tool works fine for what it does, but:

1. **Plain CSV is more token-efficient** (saves 22 tokens)
2. **You're probably better off** explaining the schema separately if tokens matter
3. **Most useful for multi-table exports** where you need clear boundaries
4. **Not revolutionary** - it's literally just CSV with a header

Use it if the header clarity is worth 22 tokens to you. Don't use it if every token counts.

## Specification

See [TOSE_SPECIFICATION.md](TOSE_SPECIFICATION.md) for complete format details.

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

82 comprehensive tests covering:
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

- **Streaming**: Temporary file buffering for constant memory usage
- **Zero-copy**: Preserves CSV data byte-for-byte (no parsing/re-encoding)
- **Performance**: Handles gigabytes of data with buffered I/O

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! This is an educational project exploring token optimization strategies.

## Links

- [GitHub Repository](https://github.com/Jeecabs/tose_converter)
- [Homebrew Tap](https://github.com/Jeecabs/homebrew-tose)
- [Issue Tracker](https://github.com/Jeecabs/tose_converter/issues)
