# CLI Guide - Check Command

The `check` command validates MON files and optionally runs the linter.

## Basic Usage

```bash
mon check <file>
```

## Options

| Flag | Description |
|------|-------------|
| `--lint` | Run linter for code quality analysis |
| `--as-json` | Output diagnostics in JSON format |

## Examples

### 1. Basic Validation

Check if a file is valid MON:

```bash
mon check config.mon
```

**Input** (`config.mon`):
```mon
{
    database: {
        host: "localhost",
        port: 5432
    }
}
```

**Output**:
```
Checking config.mon...
✓ config.mon is valid
```

### 2. Validation with Errors

```bash
mon check invalid.mon
```

**Input** (`invalid.mon`):
```mon
{
    key: value  // Missing quotes
}
```

**Output**:
```
Error: Expected a comma or a closing brace '}' after a member, but found an identifier.
  ┌─ invalid.mon:2:10
  │
2 │     key: value
  │          ^^^^^
```

### 3. Linting

Run the linter to catch code quality issues:

```bash
mon check app.mon --lint
```

**Input** (`app.mon`):
```mon
{
    &unused_anchor: { data: 123 },
    
    config: {
        timeout: 30,
        retries: 5
    }
}
```

**Output**:
```
Checking app.mon...
Linting app.mon

  [W] LINT2001 Anchor 'unused_anchor' is defined but never used

Summary: 0 errors, 1 warnings, 0 hints

Run 'mon check --explain LINT2001' for detailed information
Configure rules in .monlint.mon
```

### 4. JSON Output

Get machine-readable diagnostics:

```bash
mon check app.mon --lint --as-json
```

**Output**:
```json
{
  "diagnostics": [
    {
      "code": "LINT2001",
      "code_name": "UnusedAnchor",
      "severity": "Warning",
      "message": "Anchor 'unused_anchor' is defined but never used",
      "range": {
        "start": { "line": 1, "character": 4 },
        "end": { "line": 1, "character": 18 }
      },
      "tags": ["Unnecessary"]
    }
  ]
}
```

### 5. Multiple Files

Check multiple files in a directory:

```bash
# Using shell glob
mon check *.mon

# Or loop through files
for file in configs/*.mon; do
    mon check "$file" --lint
done
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (valid file, no errors) |
| 1 | Parse error or validation failure |
| 2 | Linter errors (if --lint is used) |

## Integration with CI/CD

### GitHub Actions

```yaml
- name: Check MON files
  run: |
    find . -name "*.mon" -exec mon check {} --lint \;
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

for file in $(git diff --cached --name-only --diff-filter=ACM | grep '\.mon$'); do
    mon check "$file" --lint || exit 1
done
```

## Tips

1. **Use --lint in CI** - Catch issues before they reach production
2. **JSON output for tooling** - Parse diagnostics programmatically
3. **Combine with fmt** - `mon fmt file.mon && mon check file.mon --lint`

## See Also

- [Linter Rules](../linter/README.md) - All available lint rules
- [Format Command](fmt.md) - Formatting MON files
- [Configuration](../linter/configuration.md) - Customize linter behavior
