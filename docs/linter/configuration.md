# Linter Configuration Reference

Complete reference for `.monlint.mon` configuration options.

## Quick Start

Create `.monlint.mon` in your project root:

```mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    max_nesting_depth: 5,
    warn_unused_anchors: true,
}
```

Or use `mon init` for interactive setup:

```bash
mon init --linter
```

## Configuration Options

### Complexity Limits (LINT1XXX)

#### `max_nesting_depth`
- **Type**: Number
- **Default**: `5`
- **Range**: `3-10`
- **Rule**: [LINT1001](complexity.md#lint1001)

Controls maximum depth of nested objects/arrays.

```mon
{
    max_nesting_depth: 3,  // Strict - enforce flat structures
}
```

#### `max_object_members`
- **Type**: Number
- **Default**: `20`
- **Range**: `5-100`
- **Rule**: [LINT1002](complexity.md#lint1002)

Maximum number of fields in an object.

```mon
{
    max_object_members: 15,  // Keep objects focused
}
```

#### `max_array_elements`
- **Type**: Number
- **Default**: `50`
- **Range**: `5-200`
- **Rule**: [LINT1003](complexity.md#lint1003)

Maximum number of elements in an array.

```mon
{
    max_array_elements: 100,  // Allow larger arrays
}
```

### Code Quality (LINT2XXX)

#### `warn_unused_anchors`
- **Type**: Boolean
- **Default**: `true`
- **Rule**: [LINT2001](smells.md#lint2001)

Warn about anchors that are never referenced.

```mon
{
    warn_unused_anchors: false,  // Disable unused anchor warnings
}
```

#### `warn_magic_numbers`
- **Type**: Boolean
- **Default**: `false`
- **Rule**: [LINT2004](smells.md#lint2004)

Suggest extracting hardcoded numbers into named constants.

```mon
{
    warn_magic_numbers: true,  // Encourage named constants
}
```

#### `max_spreads`
- **Type**: Number
- **Default**: `3`
- **Range**: `1-10`
- **Rule**: [LINT2003](smells.md#lint2003)

Maximum number of spread operations in a single object.

```mon
{
    max_spreads: 5,  // Allow more complex composition
}
```

### Best Practices (LINT3XXX)

#### `suggest_type_validation`
- **Type**: Boolean
- **Default**: `true`
- **Rule**: [LINT3001](best-practices.md#lint3001)

Suggest adding type annotations for complex structures.

```mon
{
    suggest_type_validation: false,  // Don't suggest types
}
```

#### `warn_empty_structures`
- **Type**: Boolean
- **Default**: `true`
- **Rule**: [LINT3003](best-practices.md#lint3003)

Warn about empty objects and arrays.

```mon
{
    warn_empty_structures: false,  // Allow empty structures
}
```

#### `enforce_consistent_naming`
- **Type**: Boolean
- **Default**: `false`
- **Rule**: [LINT3002](best-practices.md#lint3002)

Enforce consistent naming style within objects.

```mon
{
    enforce_consistent_naming: true,  // Enforce snake_case consistency
}
```

### Import Analysis (LINT4XXX)

#### `max_import_chain_depth`
- **Type**: Number
- **Default**: `5`
- **Range**: `2-10`
- **Rule**: [LINT4001](imports.md#lint4001)

Maximum depth of import chains.

```mon
{
    max_import_chain_depth: 3,  // Prefer shallow import hierarchies
}
```

### Advanced Options

#### `disabled_rules`
- **Type**: Array of Strings
- **Default**: `[]`

List of diagnostic codes to completely disable.

```mon
{
    disabled_rules: ["LINT2004", "LINT3001"],  // Disable specific rules
}
```

## Configuration Templates

### Strict (Production)

```mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    // Strict complexity limits
    max_nesting_depth: 3,
    max_object_members: 15,
    max_array_elements: 30,
    
    // All warnings enabled
    warn_unused_anchors: true,
    warn_magic_numbers: true,
    warn_empty_structures: true,
    
    // Enforce best practices
    suggest_type_validation: true,
    enforce_consistent_naming: true,
    
    // Tight import control
    max_spreads: 2,
    max_import_chain_depth: 3,
}
```

### Lenient (Prototyping)

```mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    // Relaxed limits
    max_nesting_depth: 7,
    max_object_members: 30,
    max_array_elements: 100,
    
    // Only critical warnings
    warn_unused_anchors: false,
    warn_magic_numbers: false,
    warn_empty_structures: false,
    
    // Suggestions off
    suggest_type_validation: false,
    enforce_consistent_naming: false,
    
    // Lenient spreads
    max_spreads: 5,
    max_import_chain_depth: 7,
}
```

### Balanced (Recommended)

```mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    // Default values work well for most projects
    max_nesting_depth: 5,
    max_object_members: 20,
    max_array_elements: 50,
    warn_unused_anchors: true,
    max_spreads: 3,
    max_import_chain_depth: 5,
}
```

## Environment-Specific Configs

### Development

```.mon
// .monlint.dev.mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    warn_magic_numbers: false,  // Allow during dev
    warn_empty_structures: false,
}
```

### Production

```mon
// .monlint.prod.mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    max_nesting_depth: 3,       // Strict
    warn_magic_numbers: true,
    suggest_type_validation: true,
}
```

Use with:
```bash
# Development
mon check *.mon --lint --config .monlint.dev.mon

# Production
mon check *.mon --lint --config .monlint.prod.mon
```

## Tips

1. **Start with defaults** - They work well for most projects
2. **Tune gradually** - Adjust based on your team's needs
3. **Use templates** - Start with `strict` or `lenient` and customize
4. **Document overrides** - Add comments explaining why you changed defaults
5. **Version control** - Commit `.monlint.mon` so team shares settings
6. **Per-directory configs** - Place `.monlint.mon` in subdirs for different rules

## See Also

- [Linter Overview](README.md) - Getting started
- [Complexity Rules](complexity.md) - LINT1XXX details
- [Code Smells](smells.md) - LINT2XXX details
- [Best Practices](best-practices.md) - LINT3XXX details
- [Import Analysis](imports.md) - LINT4XXX details
