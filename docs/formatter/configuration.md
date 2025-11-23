# Formatter Configuration

Customize MON code formatting with `.monfmt.mon`

## Quick Start

Create `.monfmt.mon` in your project root:

```mon
{
    indent_size: 4,
    max_line_width: 80,
    trailing_commas: "always"
}
```

Then format:

```bash
mon fmt file.mon --config .monfmt.mon --write
```

## Complete Configuration Reference

### Basic Options

```mon
{
    // Indentation
    indent_size: 4,              // Number of spaces per level (default: 4)
    indent_style: "spaces",      // "spaces" or "tabs" (default: "spaces")
    
    // Line width
    max_line_width: 80,          // Maximum characters per line (default: 80)
    
    // Trailing commas
    trailing_commas: "always",   // "always", "never", "multiline" (default: "always")
    
    // Object/Array formatting
    object_style: "expanded",    // "expanded" or "compact" (default: "expanded")
    array_style: "expanded",     // "expanded" or "compact" (default: "expanded")
}
```

### Advanced Options

```mon
{
    // Comment alignment
    align_trailing_comments: true,     // Align trailing comments (default: false)
    comment_alignment_column: 40,      // Column to align comments to (default: 40)
    
    // Key sorting
    sort_keys: "none",                 // "none", "alpha", "length" (default: "none")
}
```

## Examples

### Compact Style

```mon
{
    indent_size: 2,
    object_style: "compact",
    array_style: "compact",
    trailing_commas: "never"
}
```

**Result**:
```mon
{
  config: { host: "localhost", port: 5432 },
  tags: ["prod", "critical"]
}
```

### Expanded Style (Default)

```mon
{
    indent_size: 4,
    object_style: "expanded",
    array_style: "expanded",
    trailing_commas: "always"
}
```

**Result**:
```mon
{
    config: {
        host: "localhost",
        port: 5432,
    },
    tags: [
        "prod",
        "critical",
    ],
}
```

### Comment Alignment

```mon
{
    align_trailing_comments: true,
    comment_alignment_column: 40
}
```

**Result**:
```mon
{
    timeout: 30,                        // Seconds
    retries: 5,                         // Max retry attempts
    backoff: 1000,                      // Milliseconds
}
```

### Alphabetical Key Sorting

```mon
{
    sort_keys: "alpha"
}
```

**Input**:
```mon
{ z: 3, a: 1, m: 2 }
```

**Output**:
```mon
{
    a: 1,
    m: 2,
    z: 3,
}
```

## Predefined Styles

### Google Style

```bash
mon fmt file.mon --style google
```

```mon
{
    indent_size: 2,
    max_line_width: 100,
    object_style: "compact",
    trailing_commas: "multiline"
}
```

### Mozilla Style

```bash
mon fmt file.mon --style mozilla
```

```mon
{
    indent_size: 4,
    max_line_width: 100,
    object_style: "expanded",
    trailing_commas: "always"
}
```

### Linux Style

```bash
mon fmt file.mon --style linux
```

```mon
{
    indent_style: "tabs",
    object_style: "compact",
    trailing_commas: "never",
    max_line_width: 100
}
```

## Team Configuration

### 1. Commit `.monfmt.mon`

```bash
git add .monfmt.mon
git commit -m "Add MON formatter config"
```

### 2. Enforce in CI

```yaml
# .github/workflows/ci.yml
- name: Check Formatting
  run: mon fmt **/*.mon --config .monfmt.mon --check
```

### 3. Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit
mon fmt $(git diff --name-only --cached | grep '\.mon$') --write
```

## Tips

1. **Start simple** - Use defaults, adjust as needed
2. **Team consensus** - Agree on style before committing
3. **Format before commit** - Clean code in version control
4. **Use --check in CI** - Enforce formatting standards

## See Also

- [CLI - Format Command](../cli/fmt.md)
- [Coding Styles](styles.md)
- [Advanced Features](advanced.md)
