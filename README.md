# Markdown Base CLI (mdb)

A high-performance CLI tool for indexing and querying Markdown files with DuckDB. Obsidian-compatible.

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://rust-lang.org)
[![DuckDB](https://img.shields.io/badge/DuckDB-1.4+-yellow?logo=duckdb)](https://duckdb.org)

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd mdb

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
mdb index --base-dir ./my-notes

# Query notes
mdb query -q "has(tags, 'todo')"
```

## Commands

### `index`
Scans Markdown files and indexes to DuckDB.

```bash
mdb index --base-dir ./notes        # Index base directory
mdb index --base-dir ./notes --force     # Force re-index
mdb index --base-dir ./notes -v     # Verbose
```

### `query`
Query indexed files with SQL-like expressions.

```bash
# Basic queries (shorthand - native columns and frontmatter properties)
mdb query -q "has(tags, 'project')"
mdb query -q "category == 'work'"
mdb query -q "folder =~ '%projects%'"
mdb query -q "mtime > '2024-01-01'"

# Explicit namespace usage (file.* for native columns, note.* for frontmatter)
mdb query -q "has(file.tags, 'todo')"
mdb query -q "note.author == 'John'"

# Output formats
mdb query -q "has(tags, 'todo')" -o json
mdb query -q "has(tags, 'todo')" -o list

# Select fields (default: file.path, file.mtime)
mdb query -q "name == 'readme'" -f "path,name,size"
mdb query -q "category == 'project'" -f "path,note.author,category"
```

**Fields:** Native columns (`path`, `folder`, `name`, `ext`, `size`, `ctime`, `mtime`, `content`, `tags`, `links`, `backlinks`, `embeds`) and frontmatter properties (e.g., `author`, `category`). Use `file.*` prefix for explicit namespace or shorthand for convenience.

**Operators:** `==`, `!=`, `>`, `<`, `>=`, `<=`, `=~` (LIKE), `and`, `or`

**Functions:** `has(field, value)` - array containment

**Note:** Shorthand notation allows concise field names - native columns (path, folder, name, tags, etc.) resolve directly, while unknown identifiers resolve to frontmatter properties. Use explicit namespaces (`file.*`, `note.*`) when needed for clarity.

**Note:** Timestamps are displayed in human-readable format (YYYY-MM-DD HH:MM:SS)

## Features

- Fast indexing with DuckDB
- SQL-like query language
- Obsidian support (wiki-links, embeds, frontmatter, tags)
- Incremental updates
- Multiple output formats (table, json, list)
- Human-readable timestamps
- Shorthand field notation for conciseness

## Development

```bash
# Build debug version
cargo build

# Run in development
cargo run -- index --base-dir ./notes
cargo run -- query -q "file.name == 'readme'"

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Build release
cargo build --release

# Run with verbose output
cargo run -- index --base-dir ./notes -v
```

## Testing

The project includes comprehensive unit tests covering all major components:

- **97 total tests** across all modules
- **Query System**: Tokenizer, parser, compiler, and SQL generation
- **Content Extraction**: Frontmatter, tags, wiki-links, embeds
- **Database**: CRUD operations, queries, and filtering
- **Scanner**: File discovery, indexing, and backlink tracking
- **Output**: Table, JSON, and list formatting

Run tests with: `cargo test`

## Tech Stack

- **Language:** Rust 1.85+ (2024 edition)
- **CLI Framework:** clap v4.5 (derive feature)
- **Database:** DuckDB via `duckdb` crate (bundled feature)
- **File Discovery:** walkdir v2.5
- **Parser:** serde_yaml (frontmatter), regex (wiki-links/tags)
- **Serialization:** serde, serde_json

## Project Structure

```
mdb/
├── Cargo.toml           # Rust dependencies and metadata
├── Cargo.lock           # Dependency lock file
├── README.md            # User documentation
├── AGENTS.md            # This file - agent specification
├── src/
│   ├── main.rs          # CLI entry point with clap
│   ├── db.rs            # DuckDB database operations
│   ├── scanner.rs       # File discovery and indexing
│   ├── extractor.rs     # Markdown content extraction
│   ├── lib.rs           # Library exports
│   └── query/           # Query system
│       ├── mod.rs       # Output formatting (table/json/list)
│       ├── tokenizer.rs # Query tokenization
│       ├── parser.rs    # AST parsing
│       └── compiler.rs  # SQL compilation
└── target/              # Build output
```

## License

MIT
