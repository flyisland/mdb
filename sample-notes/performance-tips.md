---
title: Performance Optimization Tips
tags: [performance, optimization, tips]
priority: high
---

# Performance Optimization Tips

## Indexing Performance

### 1. Use Incremental Updates
```bash
# Only index changed files (default behavior)
mdb index -d ./notes

# Force full re-index only when necessary
mdb index -d ./notes --force
```

### 2. Organize Your Notes
- Keep related notes in subdirectories
- Use consistent naming conventions
- Avoid deeply nested folders (>5 levels)

### 3. Limit File Size
Very large markdown files (>1MB) can slow down indexing:
- Split long documents
- Move large code blocks to separate files

## Search Performance

### Query Optimization

**Fast queries:**
```bash
mdb search -t "documentation"           # Tag filter (indexed)
mdb search -f "projects"                # Folder filter (indexed)
mdb search -q "exact-match"             # Exact keyword
```

**Slower queries:**
```bash
mdb search -q "%partial%"               # Leading wildcard
mdb search -q "very long query text"    # Long text search
```

### Result Limits
Always use `--limit` for large datasets:
```bash
mdb search -q "meeting" --limit 10      # Fast: only 10 results
mdb search -q "meeting"                 # Slower: all results
```

## Database Maintenance

### Check Database Size
```bash
ls -lh docs.duckdb
```

### Rebuild Database
If database becomes corrupted or slow:
```bash
rm docs.duckdb
mdb index -d ./notes --force
```

## Benchmarks

Target performance on modern hardware:
- **10,000 files**: < 5 seconds indexing
- **Search queries**: < 50ms
- **Memory usage**: < 200MB

## Related

- See [[architecture]] for system design
- See [[troubleshooting]] for common issues

#performance #optimization #tips
