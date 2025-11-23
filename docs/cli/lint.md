# `mon lint` - Code Quality Analysis

> Run linter for code quality analysis across multiple MON files

## Synopsis

```bash
mon lint [OPTIONS] <FILES>...
```

## Description

The `mon lint` command analyzes MON files for code quality issues, detecting problems like:

- Excessive nesting depth
- Large objects/arrays
- Duplicate keys
- Unused anchors and imports
- Magic numbers
- Code smells

Unlike `mon check --lint`, this standalone command supports:

- Multi-file analysis
- Custom rule filtering
- Multiple output formats
- Config file loading

## Arguments

### `<FILES>...`

One or more MON files to lint. Supports glob patterns.

```bash
mon lint config.mon
mon lint src/**/*.mon
mon lint file1.mon file2.mon file3.mon
```

## Options

### `-f, --fix`

Auto-fix issues where possible (currently a stub for future implementation).

```bash
mon lint --fix config.mon
```

### `-c, --config <PATH>`

Path to lint configuration file. If not specified, looks for `.moncfg.mon` in current directory.

```bash
mon lint --config .monlint.mon data.mon
```

### `--format <FORMAT>`

Output format: `text` (default), `json`, or `sarif`.

```bash
# Human-readable output (default)
mon lint config.mon

# JSON for CI/CD integration
mon lint --format json src/**/*.mon

# SARIF for code analysis tools (not yet implemented)
mon lint --format sarif config.mon
```

### `--rules <RULES>`

Comma-separated list of rules to enable. Only these rules will run.

```bash
mon lint --rules unused-anchor,duplicate-key config.mon
```

> **Note**: Rule filtering is currently a stub and will be fully implemented in a future release.

### `--no-rules <RULES>`

Comma-separated list of rules to disable.

```bash
mon lint --no-rules magic-number,max-nesting config.mon
```

> **Note**: Rule filtering is currently a stub and will be fully implemented in a future release.

## Exit Codes

| Code | Meaning                             |
| ---- | ----------------------------------- |
| `0`  | No errors found (warnings are okay) |
| `4`  | Linting found one or more errors    |

## Output Formats

### Text (Default)

Human-readable output with color-coded diagnostics:

```
  ✓ config.mon

database.mon

  ERR LINT1001 Duplicate key 'host' found in object
    at line 5, column 3

  WARN LINT2001 Object has 25 members, exceeds limit of 20
    at line 1, column 1

Summary: 2 files linted
  1 file(s) with errors
  1 file(s) with warnings
```

### JSON

Structured output for programmatic consumption:

```json
[
  [
    "config.mon",
    {
      "diagnostics": []
    }
  ],
  [
    "database.mon",
    {
      "diagnostics": [
        {
          "code": "DuplicateKey",
          "severity": "Error",
          "message": "Duplicate key 'host' found in object",
          "range": {
            "start": { "line": 5, "character": 3 },
            "end": { "line": 5, "character": 7 }
          }
        }
      ]
    }
  ]
]
```

## Configuration

Create a `.moncfg.mon` file to customize linter behavior:

```mon
{
  linter: {
    max_nesting_depth: 4,
    max_object_members: 20,
    max_array_items: 100,
    warn_unused_anchors: true,
    warn_magic_numbers: false
  }
}
```

## Examples

### Lint Single File

```bash
mon lint config.mon
```

### Lint Multiple Files with Pattern

```bash
mon lint src/**/*.mon
```

### CI/CD Integration

```bash
# Exit with error code if issues found
mon lint --format json src/**/*.mon > lint-report.json
if [ $? -ne 0 ]; then
  echo "Linting failed!"
  exit 1
fi
```

### Custom Configuration

```bash
mon lint --config strict-rules.mon production/*.mon
```

### Disable Specific Rules

```bash
# Allow magic numbers in test files
mon lint --no-rules magic-number tests/**/*.mon
```

## Diagnostic Codes

See [docs/linter/rules.md](../linter/rules.md) for complete list of linter rules and their codes.

Common codes:

- `LINT1001` - Duplicate key
- `LINT2001` - Max object members exceeded
- `LINT2002` - Max array items exceeded
- `LINT2003` - Max nesting depth exceeded
- `LINT3001` - Unused anchor
- `LINT4001` - Excessive spreads

## See Also

- [`mon check`](./check.md) - Syntax validation (includes basic linting with `--lint` flag)
- [Linter Configuration](../linter/configuration.md)
- [Linter Best Practices](../linter/best-practices.md)
