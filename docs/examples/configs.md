# Example Configurations

Practical `.monlint.mon` and `.monfmt.mon` examples for different projects.

## Strict Project

For critical projects requiring high code quality.

### `.monlint.mon`

```mon
{
    // Low complexity limits
    max_nesting_depth: 3,
    max_object_members: 15,
    max_array_items: 30,
    max_import_chain_depth: 2,
    
    // Strict warnings
    warn_unused_anchors: true,
    warn_magic_numbers: true,
    suggest_type_validation: true,
    
    // Treat warnings as errors
    rule_overrides: {
        "LINT2001": "error",  // Unused anchor
        "LINT2003": "error",  // Excessive spreads
        "LINT3002": "error",  // Inconsistent naming
    }
}
```

### `.monfmt.mon`

```mon
{
    indent_size: 2,
    max_line_width: 100,
    trailing_commas: "always",
    object_style: "expanded",
    align_trailing_comments: true,
    sort_keys: "alpha"
}
```

---

## Relaxed Project

For prototyping or personal projects.

### `.monlint.mon`

```mon
{
    // Generous limits
    max_nesting_depth: 10,
    max_object_members: 50,
    max_array_items: 100,
    
    // Minimal warnings
    warn_unused_anchors: false,
    warn_magic_numbers: false,
    suggest_type_validation: false,
    
    // Disable some rules
    disabled_rules: [
        "LINT2004",  // Magic numbers
        "LINT3001",  // Type validation
        "LINT3002",  // Naming consistency
    ]
}
```

### `.monfmt.mon`

```mon
{
    indent_size: 4,
    max_line_width: 120,
    trailing_commas: "multiline",
    object_style: "compact"
}
```

---

## Configuration Files Project

For managing app configuration.

### `.monlint.mon`

```mon
{
    // Reasonable limits for config
    max_nesting_depth: 5,
    max_object_members: 30,
    max_array_items: 100,
    
    // Important for config
    warn_unused_anchors: true,
    suggest_type_validation: true,
    
    // Relax some rules
    disabled_rules: ["LINT2004"],  // Magic numbers OK in config
    
    rule_overrides: {
        "LINT2002": "error",  // Duplicate keys are critical
    }
}
```

### `.monfmt.mon`

```mon
{
    indent_size: 4,
    max_line_width: 80,
    trailing_commas: "always",
    align_trailing_comments: true,
    comment_alignment_column: 50
}
```

---

## Open Source Library

For shared, public MON schemas.

### `.monlint.mon`

```mon
{
    // Moderate limits
    max_nesting_depth: 5,
    max_object_members: 25,
    max_array_items: 50,
    max_import_chain_depth: 3,
    
    // Good defaults
    warn_unused_anchors: true,
    warn_magic_numbers: true,
    suggest_type_validation: true,
    
    // Document required
    rule_overrides: {
        "LINT3001": "warning",  // Encourage types
    }
}
```

### `.monfmt.mon`

```mon
{
    indent_size: 4,
    max_line_width: 88,  // Like Python Black
    trailing_commas: "always",
    object_style: "expanded",
    sort_keys: "alpha"  // Consistent ordering
}
```

---

## Monorepo

For large projects with multiple services.

### Root `.monlint.mon`

```mon
{
    // Shared baseline
    max_nesting_depth: 5,
    max_object_members: 20,
    max_array_items: 50,
    warn_unused_anchors: true,
    warn_magic_numbers: true
}
```

### Service-Specific (Overrides)

**`services/api/.monlint.mon`**:
```mon
{
    // Inherit from root
    // import { * } from "../../.monlint.mon"
    
    // API-specific overrides
    max_object_members: 30,  // Larger API configs OK
}
```

**`services/worker/.monlint.mon`**:
```mon
{
    // Worker-specific
    warn_magic_numbers: false,  // Timing constants OK
}
```

---

## CI/CD Templates

### Strict CI Check

```yaml
# .github/workflows/mon-check.yml
name: MON Check
on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install MON
        run: cargo install mon
      
      - name: Format Check
        run: mon fmt **/*.mon --config .monfmt.mon --check
      
      - name: Lint
        run: |
          for file in $(find . -name "*.mon"); do
            mon check "$file" --lint || exit 1
          done
```

### Report Generation

```yaml
- name: Generate Lint Report
  run: |
    mon check **/*.mon --lint --as-json > lint-report.json
    
- name: Upload Report
  uses: actions/upload-artifact@v3
  with:
    name: lint-report
    path: lint-report.json
```

---

## Pre-commit Configuration

### `.pre-commit-config.yaml`

```yaml
repos:
  - repo: local
    hooks:
      - id: mon-fmt
        name: MON Format
        entry: mon fmt
        args: [--write, --config, .monfmt.mon]
        language: system
        files: \.mon$
      
      - id: mon-lint
        name: MON Lint
        entry: mon check
        args: [--lint]
        language: system
        files: \.mon$
        pass_filenames: true
```

Install:
```bash
pip install pre-commit
pre-commit install
```

---

## Tips

1. **Start with defaults** - Copy one of these templates
2. **Iterate** - Adjust based on team feedback
3. **Document** - Comment your config choices
4. **Version control** - Commit config files
5. **CI enforcement** - Make formatting/linting required

## See Also

- [Linter Configuration](../linter/configuration.md)
- [Formatter Configuration](../formatter/configuration.md)
- [CLI Workflows](cli-workflow.md)
