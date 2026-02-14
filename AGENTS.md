# Agent Specification: Markdown Base CLI

## 1. Project Overview

The goal is to build a high-performance Command Line Interface (CLI) tool designed to scan, parse, and index Markdown files into a **DuckDB** database for instantaneous metadata searching with Obsidian compatibility in mind.

## 2. Technical Stack

- **Language**: [Rust](https://rust-lang.org) 1.85+ (2024 edition)
- **Database**: [DuckDB](https://duckdb.org) via [`duckdb`](https://docs.rs/duckdb) crate with bundled feature
- **CLI Framework**: [`clap`](https://docs.rs/clap) v4.5 with derive features
- **File Discovery**: [`walkdir`](https://docs.rs/walkdir) v2.5
- **Frontmatter Parser**: [`serde_yaml`](https://docs.rs/serde_yaml) v0.9
- **Pattern Matching**: [`regex`](https://docs.rs/regex) v1.10
- **Serialization**: [`serde`](https://docs.rs/serde) v1.0 with derive features

### 2.1 Why Rust?

The project was migrated from Bun/TypeScript to Rust for:
- **Performance**: Native compilation with zero-cost abstractions
- **Type Safety**: Compile-time guarantees with expressive type system
- **Ecosystem**: Mature crates for DuckDB, CLI parsing, and file operations
- **Distribution**: Single static binary with `cargo build --release`

## 3. Data Schema

The DuckDB local file (`.mdb/mdb.duckdb`) utilizes the following schema to support Markdown structures:

| Property   | Type    | Description                                                   |
| ---------- | ------- | ------------------------------------------------------------- |
| path       | TEXT    | Primary key - full file path                                  |
| folder     | TEXT    | Directory path                                                |
| name       | TEXT    | File name (without extension)                                 |
| ext        | TEXT    | File extension                                                |
| size       | INTEGER | File size in bytes                                            |
| ctime      | BIGINT  | Created time (Unix timestamp)                                 |
| mtime      | BIGINT  | Modified time (Unix timestamp)                                |
| content    | TEXT    | File content (without frontmatter)                            |
| tags       | TEXT    | JSON array of tags                                            |
| links      | TEXT    | JSON array of wiki-links                                      |
| backlinks  | TEXT    | JSON array of backlink files                                  |
| embeds     | TEXT    | JSON array of embeds                                          |
| properties | TEXT    | JSON object of frontmatter properties                         |

**Note**: Array/Object types stored as JSON strings for DuckDB compatibility

### 3.1 Indexes
```sql
CREATE INDEX IF NOT EXISTS idx_mtime ON documents(mtime);
CREATE INDEX IF NOT EXISTS idx_folder ON documents(folder);
CREATE INDEX IF NOT EXISTS idx_name ON documents(name);
```

## 4. Operational Requirements

### Command: `index`
- **Behavior**: Scans a target directory for `.md` files recursively.
- **Concurrency**: Sequential processing with WalkDir iterator.
- **Logic**: 
    - Perform incremental updates by comparing `mtime`.
    - Extract YAML Frontmatter using `serde_yaml`.
    - Parse wiki-links `[[link]]`, embeds `![[embed]]`, and tags `#tag` using regex.
    - Calculate backlinks (reverse link lookup) post-indexing.
    - Insert documents using parameterized queries with `duckdb::params!`.
- **Options**:
    - `-b, --base-dir <path>` - Target directory (default: current)
    - `-f, --force` - Force re-index all files (ignore mtime)
    - `-v, --verbose` - Show detailed output

### Command: `query`
- **Behavior**: Query indexed files with SQL-like expressions.
- **Functionality**: 
    - Parse SQL-like query expressions with field references, operators, and logical combinations
    - Support `file.*` namespace for file metadata (path, folder, name, ext, size, ctime, mtime)
    - Support `note.*` namespace for Markdown fields (content, tags, links, backlinks, embeds, properties)
    - Support shorthand notation for frontmatter properties (e.g., `category` → `json_extract(properties, '$.category')`)
    - Support comparison operators: `==`, `!=`, `>`, `<`, `>=`, `<=`, `=~` (pattern match)
    - Support logical operators: `and`, `or` with proper precedence
    - Support `has()` function for array containment checks
    - Compile queries to DuckDB SQL for execution
    - Timestamps displayed in human-readable format (YYYY-MM-DD HH:MM:SS)
- **Options**:
    - `-q, --query <expression>` - Query expression (required)
    - `-o, --output-format <type>` - Output: table, json, list (default: table)
    - `-l, --limit <n>` - Max results (default: 1000)
    - `-f, --output-fields <fields>` - Fields to select (default: file.path, file.mtime)

## 5. Implementation Details

### 5.1 File Discovery (scanner.rs)
Uses `walkdir` to recursively find `.md` files:
```rust
for entry in WalkDir::new(dir)
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
{
    let path = entry.path();
    if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
        // Process markdown file
    }
}
```

### 5.2 Content Parsing (extractor.rs)
Regex patterns for Obsidian-style markdown:
```rust
static WIKILINK_REGEX: &str = r"\[\[([^\]]+)\]\]";
static EMBED_REGEX: &str = r"!\[\[([^\]]+)\]\]";
static TAG_REGEX: &str = r"#[\w\-/]+";
```

Frontmatter parsing with serde_yaml:
```rust
fn parse_frontmatter(content: &str) -> (Value, String) {
    if content.starts_with("---") {
        if let Some(end_idx) = content[3..].find("---") {
            let yaml_content = &content[3..end_idx + 3];
            let remaining = &content[end_idx + 6..];
            if let Ok(props) = serde_yaml::from_str::<Value>(yaml_content) {
                return (props, remaining.trim().to_string());
            }
        }
    }
    (Value::Null, content.to_string())
}
```

### 5.3 Database Operations (db.rs)
Using `duckdb` crate with Rust API:
```rust
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }
    
    pub fn upsert_document(&self, doc: &Document) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            "INSERT OR REPLACE INTO documents ...",
            params![...],
        )?;
        Ok(())
    }
}
```

**Row Access Pattern**:
```rust
let mut stmt = self.conn.prepare("SELECT * FROM documents")?;
let mut rows = stmt.query([])?;

while let Some(row) = rows.next()? {
    let val: duckdb::types::Value = row.get(i)?;
    // Process value
}
```

### 5.4 Query System (query/)
Located in `src/query/`:

- **tokenizer.rs**: Tokenizes query expressions into tokens (Field, Operator, StringLiteral, NumberLiteral, LParen, RParen, Function, And, Or)
  ```rust
  pub enum Token {
      Field(String),
      Operator(String),
      StringLiteral(String),
      NumberLiteral(String),
      LParen, RParen,
      Function(String),
      And, Or, EOF,
  }
  ```

- **parser.rs**: Recursive descent parser building AST from tokens
  ```rust
  pub enum AstNode {
      Binary { left: Box<AstNode>, op: String, right: Box<AstNode> },
      Field(String),
      StringLiteral(String),
      NumberLiteral(String),
      FunctionCall { name: String, args: Vec<AstNode> },
      Grouping(Box<AstNode>),
  }
  ```

- **compiler.rs**: Compiles AST to DuckDB SQL with field resolution
  ```rust
  pub fn resolve_field(field: &str) -> String {
      if field.contains('.') {
          // Handle file.* and note.* namespaces
      }
      // Shorthand frontmatter properties
      format!("json_extract(properties, '$.{}')", field)
  }
  ```

- **mod.rs**: Output formatting (table, json, list)

### 5.5 CLI Entry Point (main.rs)
Using clap derive macros:
```rust
#[derive(Parser)]
#[command(name = "mdb")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = ".mdb/mdb.duckdb")]
    database: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Index { ... },
    Query { ... },
}
```

### 5.6 Query Examples
```bash
# Equality
mdb query -q "file.name == 'readme'"

# Comparison
mdb query -q "file.size > 1000"
mdb query -q "file.mtime > '2024-01-01'"

# Pattern matching
mdb query -q "file.name =~ '%test%'"

# Logical operators
mdb query -q "file.name == 'readme' and file.mtime > '2024-01-01'"
mdb query -q "file.name == 'a' or file.name == 'b'"

# Array containment
mdb query -q "has(note.tags, 'important')"

# Shorthand properties (frontmatter)
mdb query -q "category == 'project'"

# Field selection (default: file.path, file.mtime)
mdb query -q "file.name == 'readme'" -f "path,name,size"

# Output formats
mdb query -q "file.name == 'readme'" -o json
mdb query -q "file.name == 'readme'" -o list
```

## 6. Performance Goals
- **Indexing Speed**: < 5 seconds for 10,000 files (cold start).
- **Search Latency**: < 50ms for complex queries on 10,000 rows.
- **Query Latency**: < 100ms for complex query expressions.
- **Binary Size**: Optimized release builds with `strip = true` in Cargo.toml.

## 7. Constraints & Safety
- **Single Writer**: Only one `index` process can run at a time (DuckDB constraint).
- **Graceful Shutdown**: Database connection closed when `Database` struct is dropped.
- **Error Handling**: Comprehensive use of `Result` type with `?` operator for propagation.
- **Incremental Updates**: Compare `mtime` to skip unchanged files.
- **Parameterized Queries**: Query compiler uses parameterized queries to prevent SQL injection.
- **Thread Safety**: Uses `Mutex<Database>` for thread-safe access in multi-threaded contexts.

## 8. Development Commands

```bash
# Build debug version
cargo build

# Run with arguments
cargo run -- index --base-dir ./notes -v
cargo run -- query -q "has(note.tags, 'todo')" -o json

# Run tests
cargo test

# Build optimized release
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy lints
cargo clippy
```

## 9. Project Structure

```
mdb/
├── Cargo.toml           # Rust dependencies and metadata
├── Cargo.lock           # Dependency lock file
├── README.md            # User documentation
├── AGENTS.md            # This file - agent specification
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── db.rs            # Database operations
│   ├── scanner.rs       # File discovery
│   ├── extractor.rs     # Content extraction
│   └── query/
│       ├── mod.rs       # Output formatting
│       ├── tokenizer.rs # Lexical analysis
│       ├── parser.rs    # AST generation
│       └── compiler.rs  # SQL compilation
└── target/              # Build output
```

## 10. Development Status

### Completed ✅
- Core indexing functionality
- Query system (SQL-like expressions)
- Field-based queries (file.*, note.*, shorthand)
- Query operators (==, !=, >, <, >=, <=, =~)
- Logical operators (and, or) with precedence
- has() function for array containment
- Multiple output formats (table, json, list)
- Backlink tracking
- Rust migration complete
- CLI with clap derive macros
- Incremental updates via mtime comparison

### Technical Debt / Future Improvements
- ✅ ~~Add unit tests for tokenizer, parser, and compiler~~ (Completed - 90 tests added)
- Add integration tests for full query pipeline
- Benchmark performance against 10,000 files goal
- Consider parallel processing for indexing
- Add configuration file support
- Implement query result caching

### Test Coverage Summary

**Unit Tests Implemented (100 tests total):**

| Module | Tests | Coverage |
|--------|-------|----------|
| `tokenizer.rs` | 13 | Field tokenization, operators, literals, functions, parentheses |
| `parser.rs` | 13 | Expression parsing, operators, grouping, precedence |
| `compiler.rs` | 17 | SQL generation, field resolution, all operators |
| `extractor.rs` | 17 | Frontmatter, tags, wiki-links, embeds, edge cases |
| `db.rs` | 8 | Database operations, queries, CRUD |
| `scanner.rs` | 13 | File scanning, indexing, backlinks, subdirectories |
| `query/mod.rs` | 9 | Output formatting (table, JSON, list) |
| `main.rs` | 10 | CLI options, default values, parsing |

**Test Execution:**
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module
cargo test tokenizer::tests
cargo test extractor::tests
```
