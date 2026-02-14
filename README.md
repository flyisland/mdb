# Markdown Base CLI (mdb)

A high-performance CLI tool for indexing and querying Markdown files with DuckDB. Obsidian-compatible.

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://rust-lang.org)
[![DuckDB](https://img.shields.io/badge/DuckDB-1.4+-yellow?logo=duckdb)](https://duckdb.org)

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd mdb-rs

# Build release binary
cargo build --release

# The binary will be at target/release/mdb
./target/release/mdb --help
```

### Prerequisites

- Rust 1.85+ (2024 edition)
- DuckDB (bundled with the `duckdb` crate)

## Quick Start

```bash
# Index notes
mdb index -d ./my-notes

# Query notes
mdb query -q "has(note.tags, 'todo')"
```

## Commands

### `index`
Scans Markdown files and indexes to DuckDB.

```bash
mdb index -d ./notes        # Index directory
mdb index -d ./notes -f     # Force re-index
mdb index -d ./notes -v     # Verbose
```

### `query`
Query indexed files with SQL-like expressions.

```bash
# Basic queries
mdb query -q "has(note.tags, 'project')"
mdb query -q "file.folder =~ '%projects%'"
mdb query -q "file.mtime > '2024-01-01'"

# Output formats
mdb query -q "has(note.tags, 'todo')" --format json
mdb query -q "has(note.tags, 'todo')" --format list

# Select fields
mdb query -q "file.name == 'readme'" -F "path,name,size"
```

**Fields:** `file.path`, `file.folder`, `file.name`, `file.ext`, `file.size`, `file.ctime`, `file.mtime`, `note.content`, `note.tags`, `note.links`, `note.backlinks`, `note.embeds`, `note.properties`

**Operators:** `==`, `!=`, `>`, `<`, `>=`, `<=`, `=~` (LIKE), `and`, `or`

**Functions:** `has(field, value)` - array containment

## Features

- Fast indexing with DuckDB
- SQL-like query language
- Obsidian support (wiki-links, embeds, frontmatter, tags)
- Incremental updates
- Multiple output formats (table, json, list)

## Development

```bash
# Build debug version
cargo build

# Run in development
cargo run -- index -d ./notes
cargo run -- query -q "file.name == 'readme'"

# Run tests
cargo test

# Build release
cargo build --release

# Run with verbose output
cargo run -- index -d ./notes -v
```

## Tech Stack

- **Language:** Rust 1.85+ (2024 edition)
- **CLI Framework:** clap v4.5 (derive feature)
- **Database:** DuckDB via `duckdb` crate (bundled feature)
- **File Discovery:** walkdir v2.5
- **Parser:** serde_yaml (frontmatter), regex (wiki-links/tags)
- **Serialization:** serde, serde_json

## Project Structure

```
src/
├── main.rs        # CLI entry point with clap
├── db.rs          # DuckDB database operations
├── scanner.rs     # File discovery and indexing
├── extractor.rs   # Markdown content extraction
├── lib.rs         # Library exports
└── query/         # Query system
    ├── mod.rs     # Output formatting (table/json/list)
    ├── tokenizer.rs  # Query tokenization
    ├── parser.rs     # AST parsing
    └── compiler.rs   # SQL compilation
```

## License

MIT
