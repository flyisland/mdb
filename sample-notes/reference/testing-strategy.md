---
title: Testing Strategy
tags: [documentation, development, testing]
category: reference
---

# Testing Strategy

## Overview

The project uses Rust's built-in testing framework with comprehensive unit tests across all modules.

## Test Organization

### Test Modules
Each source file contains a `#[cfg(test)]` module at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // Test implementation
    }
}
```

### Test Coverage by Module

| Module | Test Count | Key Areas |
|--------|-----------|-----------|
| `tokenizer.rs` | 13 | Fields, operators, literals, functions |
| `parser.rs` | 13 | Expressions, precedence, grouping |
| `compiler.rs` | 17 | SQL generation, field resolution |
| `extractor.rs` | 17 | Frontmatter, tags, links, embeds |
| `db.rs` | 8 | CRUD operations, queries |
| `scanner.rs` | 13 | File discovery, indexing |
| `query/mod.rs` | 9 | Output formatting |

## Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific module
cargo test tokenizer::tests
```

## Test Patterns

### Tokenizer Tests
- Test individual token types
- Test operator recognition
- Test complex query tokenization

### Parser Tests
- Test AST generation
- Test operator precedence
- Test grouping expressions

### Compiler Tests
- Test SQL generation
- Test field resolution (file.*, note.*, shorthand)
- Test all operators

### Database Tests
- Use temporary files with unique IDs
- Clean up after tests
- Test CRUD operations

## Future Improvements

- Integration tests for end-to-end workflows
- Performance benchmarks
- Property-based testing with proptest

#testing #development #documentation
