# Code Smells (LINT2XXX)

Common mistakes and antipatterns that should be avoided in MON files.

## LINT2001: Unused Anchor

**Severity**: Warning  
**Category**: Code Smells

### What It Does

Detects anchors (`&name`) that are defined but never referenced (`*name`).

### Problem

```mon
{
    &temp_data: {         // ⚠️ Warning: 'temp_data' is never used
        value: 123
    },
    
    config: {
        timeout: 3600
    }
}
```

### Why It Matters

- **Dead Code**: Unused anchors clutter your configuration
- **Maintenance Burden**: Readers wonder if it's intentional or forgotten
- **Potential Bug**: Might indicate incomplete refactoring

### Solution

#### Option 1: Use the Anchor

```mon
{
    &temp_data: {
        value: 123
    },
    
    config: {
        data: *temp_data   // ✅ Now it's used
    }
}
```

#### Option 2: Remove It

```mon
{
    config: {              // ✅ Removed unused anchor
        timeout: 3600
    }
}
```

### Configuration

```mon
// .mon lint.mon
{
    warn_unused_anchors: false,  // Disable this rule
}
```

---

## LINT2002: Duplicate Key

**Severity**: Error  
**Category**: Code Smells

### What It Does

Detects duplicate keys in the same object.

### Problem

```mon
{
    timeout: 1000,
    enabled: true,
    timeout: 3000,    // ❌ Error: Duplicate key 'timeout'
}
```

### Why It Matters

- **Ambiguous**: Which value should be used?
- **Error-Prone**: Different JSON/YAML parsers handle this differently
- **Usually a Mistake**: Often copy-paste errors

### Solution

```mon
{
    timeout: 3000,     // ✅ Keep only one
    enabled: true
}
```

Or if you need different timeouts:

```mon
{
    connection_timeout: 1000,
    request_timeout: 3000,
    enabled: true
}
```

### Configuration

This rule cannot be disabled - duplicate keys are always errors.

---

## LINT2003: Excessive Spreads

**Severity**: Warning  
**Category**: Code Smells

### What It Does

Warns when an object has more than 3 spread operations.

### Problem

```mon
{
    config: {
        ...*base,
        ...*overrides1,
        ...*overrides2,
        ...*overrides3,
        ...*overrides4,    // ⚠️  Warning: 5 spreads makes this hard to follow
        custom: "value"
    }
}
```

### Why It Matters

- **Hard to Understand**: Tracking the final value requires mental gymnastics
- **Merge Conflicts**: Which value wins?
- **Maintenance**: Changes in any spread affect the final result

### Solution

#### Option 1: Consolidate Spreads

```mon
{
    &merged_base: {
        ...*base,
        ...*overrides1,
        ...*overrides2,
    },
    
    config: {
        ...*merged_base,   // ✅ Only 2 spreads now
        ...*final_overrides,
        custom: "value"
    }
}
```

#### Option 2: Be Explicit

```mon
{
    config: {              // ✅ Explicit is better
        host: "localhost",
        port: 8080,
        timeout: 3000,
        custom: "value"
    }
}
```

### Configuration

```mon
// .monlint.mon
{
    max_spreads: 5,  // Allow up to 5 spreads (default: 3)
}
```

---

## LINT2004: Magic Number

**Severity**: Info  
**Category**: Code Smells

### What It Does

Suggests extracting hardcoded numbers into named constants.

### Problem

```mon
{
    timeout: 3600,          // ℹ️ What does 3600 mean?
    max_retries: 5,
    buffer_size: 8192
}
```

### Why It Matters

- **Readability**: Numbers without context are hard to understand
- **Maintenance**: Changing the same value in multiple places
- **Magic**: What does 3600 represent? Seconds? Milliseconds?

### Solution

```mon
{
    &ONE_HOUR_IN_SECONDS: 3600,
    &DEFAULT_RETRIES: 5,
    &BUFFER_8KB: 8192,
    
    timeout: *ONE_HOUR_IN_SECONDS,      // ✅ Clear intent
    max_retries: *DEFAULT_RETRIES,
    buffer_size: *BUFFER_8KB
}
```

### Exceptions

Common values are ignored:
- `0`, `1`, `-1` (obvious meanings)
- `10`, `100`, `1000` (round numbers)

### Configuration

```mon
// .monlint.mon
{
    warn_magic_numbers: false,  // Disable magic number warnings
}
```

---

## Summary

| Code | Rule | Severity | Configurable |
|------|------|----------|--------------|
| LINT2001 | UnusedAnchor | Warning | Yes (`warn_unused_anchors`) |
| LINT2002 | DuplicateKey | Error | No (always enforced) |
| LINT2003 | Excessive Spreads | Warning | Yes (`max_spreads`) |
| LINT2004 | MagicNumber | Info | Yes (`warn_magic_numbers`) |

## Quick Config

```mon
// .monlint.mon - Disable all code smell warnings
{
    warn_unused_anchors: false,
    warn_magic_numbers: false,
    max_spreads: 100,  // Effectively disabled
    
    // Can't disable duplicate keys (always an error)
}
```

## See Also

- [Complexity Rules](complexity.md) - LINT1XXX
- [Best Practices](best-practices.md) - LINT3XXX  
- [Import Analysis](imports.md) - LINT4XXX
