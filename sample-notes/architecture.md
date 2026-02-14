---
title: System Architecture
tags: [technical, design, architecture]
author: Tech Team
difficulty: advanced
---

# System Architecture

## Overview

The Markdown Base CLI uses a modern Rust architecture optimized for speed and compatibility.

### Technology Stack

```
┌─────────────────┐
│   CLI Layer     │  <- clap v4.5 (derive macros)
│  (main.rs)      │
├─────────────────┤
│  Business Logic │  <- scanner.rs, extractor.rs
│   (src/*.rs)    │
├─────────────────┤
│  Query System   │  <- tokenizer, parser, compiler
│  (src/query/)   │
├─────────────────┤
│  Database Layer │  <- duckdb crate (bundled)
│   (db.rs)       │
└─────────────────┘
```

## Key Components

### 1. File Scanner (scanner.rs)
Uses `walkdir` for efficient directory traversal:
- Recursive `.md` file discovery
- File stats extraction (size, mtime, ctime)
- Incremental updates via mtime comparison
- Backlink calculation post-indexing

### 2. Content Extractor (extractor.rs)
Combines `serde_yaml` with regex patterns:
- YAML frontmatter parsing
- Wiki-link extraction: `[[note-name]]`
- Embed detection: `![[image.png]]`
- Tag parsing: `#tag` or `#tag/subtag`

### 3. Query System (src/query/)
Three-stage query processing:
- **Tokenizer**: Lexical analysis into tokens
- **Parser**: Recursive descent AST generation
- **Compiler**: SQL generation with field resolution

### 4. Database Layer (db.rs)
DuckDB via `duckdb` crate:
- Type-safe parameterized queries
- JSON storage for arrays/objects
- Connection pooling support
- Transaction support

### 5. Testing
Comprehensive unit test coverage (90 tests):
- Query system tests (tokenizer, parser, compiler)
- Content extraction tests
- Database operation tests
- File scanning tests
- Output formatting tests

## Performance Considerations

- **Zero-cost abstractions**: Rust's compile-time optimizations
- **Sequential processing**: WalkDir iterator for memory efficiency
- **Incremental Updates**: Only re-index changed files (mtime check)
- **Database Indexes**: Optimized for search queries on mtime, folder, name

## Trade-offs

We chose Rust + DuckDB because:
- Native compilation for maximum performance
- Memory safety without garbage collection
- Better analytical query performance vs SQLite
- Single static binary for easy distribution

Arrays are stored as JSON strings for DuckDB compatibility.

## Testing Architecture

Tests are organized in `#[cfg(test)]` modules within each source file:
- **Unit tests**: Individual function testing
- **Integration tests**: Component interaction testing
- **Test isolation**: Unique temp files per test with atomic counters

See also:
- [[testing-strategy]] for detailed testing documentation
- [[performance-tips]] for optimization guide
- [[readme]] for user documentation

#technical #design #architecture #advanced
