# Linter Overview

The MON linter analyzes your code for potential issues, bad practices, and complexity problems.

## Quick Start

```bash
# Run linter
mon check file.mon --lint

# Get JSON output
mon check file.mon --lint --as-json

# Explain a rule
mon check --explain LINT2001
```

## Rule Categories

### LINT1XXX - Complexity
Issues related to code complexity and maintainability.

| Code | Rule | Severity |
|------|------|----------|
| [LINT1001](complexity.md#lint1001) | MaxNestingDepth | Warning |
| [LINT1002](complexity.md#lint1002) | MaxObjectMembers | Warning |
| [LINT1003](complexity.md#lint1003) | MaxArrayItems | Warning |

### LINT2XXX - Code Smells
Common mistakes and antipatterns.

| Code | Rule | Severity |
|------|------|----------|
| [LINT2001](smells.md#lint2001) | UnusedAnchor | Warning |
| [LINT2002](smells.md#lint2002) | DuplicateKey | Error |
| [LINT2003](smells.md#lint2003) | ExcessiveSpreads | Warning |
| [LINT2004](smells.md#lint2004) | MagicNumber | Info |

### LINT3XXX - Best Practices
Recommendations for better MON code.

| Code | Rule | Severity |
|------|------|----------|
| [LINT3001](best-practices.md#lint3001) | MissingTypeValidation | Info |
| [LINT3002](best-practices.md#lint3002) | InconsistentNaming | Warning |
| [LINT3003](best-practices.md#lint3003) | EmptyObject | Info |

### LINT4XXX - Import Analysis
Issues with imports and dependencies.

| Code | Rule | Severity |
|------|------|----------|
| [LINT4001](imports.md#lint4001) | DeepImportChain | Warning |
| [LINT4002](imports.md#lint4002) | CircularDependency | Error |
| [LINT4003](imports.md#lint4003) | UnusedImport | Warning |

## Example Output

### Human-Readable

```bash
mon check example.mon --lint
```

```
Checking example.mon...
Linting example.mon

  [W] LINT2001 Anchor 'temp_data' is defined but never used
      → Line 5:4-5:13
  
  [E] LINT2002 Duplicate key 'timeout' in object
      → Line 10:8
      → Also defined at line 8:8
  
  [I] LINT2004 Magic number 3600 without context
      → Line 15:18
      → Consider defining as a named constant

Summary: 1 error, 1 warning, 1 hint

Run 'mon check --explain <CODE>' for detailed information
Configure rules in .monlint.mon
```

### JSON Output

```bash
mon check example.mon --lint --as-json
```

```json
{
  "diagnostics": [
    {
      "code": "LINT2001",
      "code_name": "UnusedAnchor",
      "severity": "Warning",
      "message": "Anchor 'temp_data' is defined but never used",
      "range": {
        "start": { "line": 4, "character": 4 },
        "end": { "line": 4, "character": 13 }
      },
      "tags": ["Unnecessary"]
    },
    {
      "code": "LINT2002",
      "code_name": "DuplicateKey",
      "severity": "Error",
      "message": "Duplicate key 'timeout' in object",
      "range": {
        "start": { "line": 9, "character": 8 },
        "end": { "line": 9, "character": 15 }
      },
      "related_information": [
        {
          "location": {
            "uri": "file:///path/to/example.mon",
            "range": {
              "start": { "line": 7, "character": 8 },
              "end": { "line": 7, "character": 15 }
            }
          },
          "message": "First defined here"
        }
      ]
    }
  ]
}
```

## Configuration

Create `.monlint.mon` in your project root:

```mon
{
    // Complexity limits
    max_nesting_depth: 5,
    max_object_members: 20,
    max_array_items: 50,
    max_import_chain_depth: 3,
    
    // Warnings
    warn_unused_anchors: true,
    warn_magic_numbers: true,
    
    // Best practices
    suggest_type_validation: true,
    
    // Disable specific rules
    disabled_rules: ["LINT2004"],  // Disable magic number warnings
    
    // Adjust severities
    rule_overrides: {
        "LINT2001": "error",  // Make unused anchors an error
        "LINT3002": "info",   // Downgrade naming inconsistency  
    },
}
```

## Inline Suppressions

```mon
{
    // mon-lint-disable LINT2004
    timeout: 3600,  // Ok, suppressed
    
    // mon-lint-disable-next-line LINT2001
    &unused: { data: 1 },
    
    /* mon-lint-disable */ 
    config: {
        // All rules disabled in this block
    },
    /* mon-lint-enable */
}
```

## CI/CD Integration

### Fail on Errors

```bash
mon check *.mon --lint || exit 1
```

### Fail on Warnings Too

```bash
mon check *.mon --lint --strict || exit 1
```

### Generate Report

```bash
mon check *.mon --lint --as-json > lint-report.json
```

## Tips

1. **Start with defaults** - The default rules catch most issues
2. **Tune for your project** - Adjust limits in `.monlint.mon`
3. **Explain rules** - Use `mon check --explain <CODE>` to learn
4. **Progressive adoption** - Disable rules, fix gradually, re-enable
5. **Use JSON in tools** - Parse diagnostics programmatically

## See Also

- [Complexity Rules](complexity.md) - LINT1XXX details
- [Code Smells](smells.md) - LINT2XXX details
- [Best Practices](best-practices.md) - LINT3XXX details
- [Import Analysis](imports.md) - LINT4XXX details
- [Configuration Guide](configuration.md) - Complete config reference
