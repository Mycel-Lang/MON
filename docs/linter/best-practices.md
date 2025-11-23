# Best Practices (L INT3XXX)

Recommendations for writing better, more maintainable MON code.

## LINT3001: Missing Type Validation

**Severity**: Info  
**Category**: Best Practices

### What It Does

Suggests adding type validation (`:: Type`) for complex objects to catch errors early.

### Problem

```mon
{
    user: {               // ℹ️ Consider adding type validation
        id: 123,
        name: "Alice",
        email: "alice@example.com"
    }
}
```

### Why It Matters

- **Type Safety**: Catches typos and wrong types at parse time
- **Documentation**: Types serve as inline documentation
- **Refactoring**: Easier to change structure safely
- **LSP Support**: Better autocomplete and hover hints

### Solution

```mon
{
    User: #struct {
        id(Number),
        name(String),
        email(String)
    },
    
    user :: User = {      // ✅ Type-validated
        id: 123,
        name: "Alice",
        email: "alice@example.com"
    }
}
```

Now you get errors for mistakes:

```mon
{
    user :: User = {
        id: "123",        // ❌ Error: Expected Number, got String
        name: "Alice"
        // ❌ Error: Missing required field 'email'
    }
}
```

### Configuration

```mon
// .monlint.mon
{
    suggest_type_validation: false,  // Disable this suggestion
}
```

---

## LINT3002: Inconsistent Naming

**Severity**: Warning  
**Category**: Best Practices

### What It Does

Warns when an object mixes different naming conventions (snake_case, camelCase).

### Problem

```mon
{
    timeout_seconds: 30,      // snake_case
    maxRetries: 5,            // camelCase  ⚠️ Inconsistent!
    error_handler: "default",
    debugMode: true
}
```

### Why It Matters

- **Consistency**: Makes code easier to read
- **Team Standards**: Enforces agreed-upon style
- **Professionalism**: Shows attention to detail

### Solution

#### Option 1: Use snake_case (Recommended for MON)

```mon
{
    timeout_seconds: 30,      // ✅ Consistent snake_case
    max_retries: 5,
    error_handler: "default",
    debug_mode: true
}
```

#### Option 2: Use camelCase

```mon
{
    timeoutSeconds: 30,       // ✅ Consistent camelCase
    maxRetries: 5,
    errorHandler: "default",
    debugMode: true
}
```

### Configuration

```mon
// .monlint.mon
{
    enforce_consistent_naming: false,  // Disable naming checks
}
```

---

## LINT3003: Empty Object/Array

**Severity**: Info  
**Category**: Best Practices

### What It Does

Flags empty objects and arrays to verify they're intentional.

### Problem

```mon
{
    config: {},           // ℹ️ Empty object - is this intentional?
    items: [],            // ℹ️ Empty array - is this intentional?
    enabled: true
}
```

### Why It Matters

- **Incomplete Code**: Often placeholders that were forgotten
- **Dead Code**: Might indicate unused configuration
- **Clarity**: Explicit is better than implicit

### Solution

#### Option 1: Add TODO Comment

```mon
{
    config: {},           // TODO: Add configuration options
    items: [],            // Will be populated by loader
    enabled: true
}
```

#### Option 2: Remove If Unused

```mon
{
    enabled: true         // ✅ Removed empty structures
}
```

#### Option 3: Use Explicit Null

```mon
{
    config: null,         // ✅ Explicitly no config
    items: null,
    enabled: true
}
```

### Configuration

```mon
// .monlint.mon
{
    warn_empty_structures: false,  // Allow empty objects/arrays
}
```

---

## Summary

| Code | Rule | Severity | Configurable |
|------|------|----------|--------------|
| LINT3001 | MissingTypeValidation | Info | Yes (`suggest_type_validation`) |
| LINT3002 | InconsistentNaming | Warning | Yes (`enforce_consistent_naming`) |
| LINT3003 | EmptyObject | Info | Yes (`warn_empty_structures`) |

## Quick Config

```mon
// .monlint.mon - Disable all best practice suggestions
{
    suggest_type_validation: false,
    enforce_consistent_naming: false,
    warn_empty_structures: false,
}
```

## Recommended Setup

For production code, enable these for better quality:

```mon
// .monlint.mon - Production settings
{
    suggest_type_validation: true,   // Catch errors early
    enforce_consistent_naming: true, // Consistent style
    warn_empty_structures: true,     // No accidental placeholders
}
```

## See Also

- [Complexity Rules](complexity.md) - LINT1XXX
- [Code Smells](smells.md) - LINT2XXX
- [Import Analysis](imports.md) - LINT4XXX
