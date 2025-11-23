# Formatter Enhancement Implementation

## Priority 1: Full Comment Preservation

### Current State
- Comments are extracted with positions
- NOT YET reintegrated into output

### Implementation Plan
1. Build a comment map keyed by line number
2. During formatting, check for comments before/after each element
3. Insert comments at appropriate locations:
   - Standalone comments on own line
   - Trailing comments at end of line
4. Handle inline comments (between tokens)

### Approach
- Track source span of each formatted element
- Map comments to nearest AST node
- Preserve original comment placement where possible
- Options: preserve, move-to-end-of-line, move-to-own-line

## Priority 2: Proper Struct/Enum Formatting

### Struct Formatting
```mon
User: #struct {
    id(Number),
    name(String),
    email(String) = null,
}
```

### Enum Formatting
```mon
Status: #enum {
    Pending,
    Running,
    Completed,
    Failed,
}
```

### Implementation
- Format field definitions with type hints
- Handle default values
- Proper indentation and spacing
- Trailing commas configuration

## Priority 3: Line Width Enforcement

### Strategy
- Track current line width during formatting
- Break long lines intelligently:
  - Arrays: break after comma
  - Objects: break after comma
  - Long strings: don't break (user should handle)
- Configurable max_line_width (default: 80)

### Breaking Rules
1. Always break if line would exceed max_line_width
2. Keep small collections on one line if under threshold
3. Prefer breaking at natural boundaries (commas, braces)

## Priority 4: Alignment Options

### Trailing Comment Alignment
```mon
{
    key1: "value",      // Comment 1
    longer_key: "val",  // Comment 2
}
```

### Value Alignment (optional)
```mon
{
    short:  "value",
    longer: "value",
}
```

### Config Options
- `align_trailing_comments`: column number (e.g., 40)
- `align_values`: boolean

## Priority 5: Additional Configuration

### New Options
- `sort_keys`: alphabetically sort object keys
- `import_order`: control import statement ordering
- `blank_lines_between_imports_and_code`: number of blank lines

### Implementation Status
- [x] Basic config structure
- [ ] Sort keys option
- [ ] Import ordering
- [ ] Blank line control
