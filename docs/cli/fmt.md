# CLI Guide - Format Command

The `fmt` command formats MON files according to style rules.

## Basic Usage

```bash
mon fmt <file>
```

## Options

| Flag | Description |
|------|-------------|
| `--check` | Check if file is formatted (don't modify) |
| `--write` | Write formatted output to file |
| `--config <path>` | Path to .monfmt.mon config file |
| `--style <name>` | Use a predefined style (google, mozilla, etc.) |
| `--watch` | Watch for changes and auto-format |

## Examples

### 1. Format and Print

Format a file and print to stdout:

```bash
mon fmt config.mon
```

**Input**:
```mon
{a:1,b:2,c:3}
```

**Output**:
```mon
{
    a: 1,
    b: 2,
    c: 3,
}
```

### 2. Format In-Place

Modify the file directly:

```bash
mon fmt config.mon --write
```

**Before** (`config.mon`):
```mon
{
database:{host:"localhost",port:5432},
cache:{ttl:3600}
}
```

**After** (`config.mon`):
```mon
{
    database: {
        host: "localhost",
        port: 5432,
    },
    cache: {
        ttl: 3600,
    },
}
```

### 3. Check Formatting

Check if a file is already formatted (CI usage):

```bash
mon fmt config.mon --check
```

**Exit codes**:
- `0` - File is formatted correctly
- `1` - File needs formatting

**Usage in CI**:
```bash
# Fail if any file is not formatted
mon fmt *.mon --check || echo "Run 'mon fmt --write' to fix"
```

### 4. Use Predefined Style

```bash
mon fmt server.mon --style google
```

**Available styles**:
- `google` - Google style guide (2 spaces, compact)
- `mozilla` - Mozilla style (4 spaces, relaxed)
- `linux` - Linux kernel style (tabs, compact)
- `default` - MON default style

### 5. Custom Configuration

Create `.monfmt.mon`:

```mon
{
    indent_size: 4,
    max_line_width: 100,
    trailing_commas: "always",
    object_style: "expanded",
    
    // Advanced options
    align_trailing_comments: true,
    comment_alignment_column: 40,
    sort_keys: "none",  // or "alpha" or "length"
}
```

Then format with:

```bash
mon fmt app.mon --config .monfmt.mon --write
```

### 6. Watch Mode

Auto-format files on save (great for development):

```bash
mon fmt src/ --watch
```

**Output**:
```
Watching for changes in src/...
  ✓ Formatted src/config.mon
  ✓ Formatted src/schema.mon
Watching...
```

Press `Ctrl+C` to stop.

### 7. Format Multiple Files

```bash
# Format all MON files
mon fmt **/*.mon --write

# Format specific directory
mon fmt configs/*.mon --write
```

## Formatting Rules

### Indentation

```mon
// Before
{a:{b:{c:1}}}

// After
{
    a: {
        b: {
            c: 1,
        },
    },
}
```

### Comments

```mon
// Preserved and properly indented
{
    // Configuration options
    timeout: 30,  // In seconds
    
    // Database settings
    database: {
        host: "localhost",
    },
}
```

### Anchors and Aliases

```mon
// Properly formatted
{
    &base: {
        id: 1,
        name: "Base",
    },
    
    extended: {
        ...*base,
        extra: "data",
    },
}
```

### Type Definitions

```mon
// Structured formatting
{
    User: #struct {
        id(Number),
        name(String),
        email(String),
        created_at(String) = "2024-01-01",
    },
    
    Status: #enum {
        Active,
        Inactive,
        Pending,
    },
}
```

## Configuration Options

### Basic Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `indent_size` | Number | 4 | Spaces per indent level |
| `max_line_width` | Number | 80 | Maximum line width |
| `trailing_commas` | String | "always" | "always", "never", "multiline" |

### Object/Array Style

| Option | Values | Description |
|--------|--------|-------------|
| `object_style` | `"compact"`, `"expanded"` | Object formatting |
| `array_style` | `"compact"`, `"expanded"` | Array formatting |

### Advanced Options

| Option | Type | Description |
|--------|------|-------------|
| `align_trailing_comments` | Boolean | Align comments to column |
| `comment_alignment_column` | Number | Column for comment alignment |
| `sort_keys` | String | "none", "alpha", "length" |

## Integration Examples

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.mon$')

if [ -n "$FILES" ]; then
    for file in $FILES; do
        mon fmt "$file" --write
        git add "$file"
    done
fi
```

### Editor Integration

#### VSCode (tasks.json)
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Format MON",
      "type": "shell",
      "command": "mon fmt ${file} --write"
    }
  ]
}
```

#### Vim
```vim
" Format on save
autocmd BufWritePre *.mon !mon fmt % --write
```

## Tips

1. **Use --check in CI** - Enforce formatting standards
2. **Watch mode for dev** - Auto-format while coding
3. **Commit .monfmt.mon** - Share team formatting rules
4. **Format before linting** - Clean code first, then lint

## See Also

- [Formatter Configuration](../formatter/configuration.md) - All options
- [Coding Styles](../formatter/styles.md) - Predefined styles
- [Advanced Features](../formatter/advanced.md) - Key sorting, comment alignment
