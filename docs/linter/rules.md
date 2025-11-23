# MON Lint Rules Reference

Complete reference for all lint rules, their diagnostic codes, and how to configure them.

## Quick Reference

| Code | Severity | Rule | Default |
|------|----------|------|---------|
| LINT1001 | Warning | Excessive nesting depth | 4 levels |
| LINT1002 | Warning | Too many object members | 20 members |
| LINT1003 | Warning | Too many array items | 100 items |
| LINT2001 | Warning | Unused anchor definition | Enabled |
| LINT2002 | Error | Duplicate object key | Always on |
| LINT2003 | Warning | Too many spread operators | 3 spreads |
| LINT2004 | Info | Magic number literal | Disabled |
| LINT3001 | Info | Missing type validation | Disabled |
| LINT3002 | Info | Inconsistent naming convention | Disabled |
| LINT3003 | Info | Empty object or array | Disabled |
| LINT4001 | Warning | Deep import chain | 2 levels |
| LINT4002 | Error | Circular dependency | Always on |
| LINT4003 | Warning | Unused import | Disabled |

## Configuration

Create a `.monlint.mon` file in your project root:

```mon
{
    // Complexity thresholds
    max_nesting_depth: 4,
    max_object_members: 20,
    max_array_items: 100,
    max_spreads_per_object: 3,
    max_import_chain_depth: 2,
    
    // Warning toggles  
    warn_unused_anchors: true,
    warn_magic_numbers: false,
    warn_unused_imports: false,
    warn_empty_structures: false,
    
    // Best practice suggestions
    suggest_type_validation: false,
    enforce_naming_convention: false,
}
```

## Lint Rules Detailed Documentation

### LINT1001: Excessive Nesting Depth

**Severity**: Warning  
**Category**: Complexity  
**Default Limit**: 4 levels

**Description**

Deeply nested structures (objects within objects within objects) are difficult to read, understand, and maintain. This rule warns when your nesting depth exceeds the configured limit.

**Problematic Code**

```mon
{
    level1: {
        level2: {
            level3: {
                level4: {
                    level5: {  // LINT1001: Nesting depth of 5 exceeds limit of 4
                        data: "too deep"
                    }
                }
            }
        }
    }
}
```

**Recommended Solution**

1. Extract nested structures into separate anchors:
```mon
{
    &deep_config: { data: "value" },
    
    level1: {
        level2: {
            level3: *deep_config  // Reference instead of nesting
        }
    }
}
```

2. Flatten the structure if possible:
```mon
{
    config_data: "value",
    config_level: "level5"
}
```

**Configuration**

```mon
{
    max_nesting_depth: 6  // Increase to 6 if needed
}
```

---

### LINT1002: Too Many Object Members

**Severity**: Warning  
**Category**: Complexity  
**Default Limit**: 20 members

**Description**

Objects with too many members become difficult to understand at a glance. This suggests splitting into focused, smaller objects.

**Problematic Code**

```mon
{
    user: {
        id: 1,
        name: "Alice",
        email: "alice@example.com",
        phone: "555-0100",
        address_line1: "123 Main St",
        address_line2: "Apt 4B",
        city: "Springfield",
        state: "IL",
        zip: "62701",
        country: "USA",
        // ... 15 more fields  // LINT1002: Object has 25 members
    }
}
```

**Recommended Solution**

Split into related groups:
```mon
{
    user: {
        id: 1,
        name: "Alice",
        contact: {
            email: "alice@example.com",
            phone: "555-0100"
        },
        address: {
            line1: "123 Main St",
            line2: "Apt 4B",
            city: "Springfield",
            state: "IL",
            zip: "62701",
            country: "USA"
        }
    }
}
```

**Configuration**

```mon
{
    max_object_members: 30  // Increase if your use case requires it
}
```

---

### LINT1003: Too Many Array Items

**Severity**: Warning  
**Category**: Complexity  
**Default Limit**: 100 items

**Description**

Very large arrays in configuration files may indicate a need for external data storage or pagination.

**Problematic Code**

```mon
{
    items: [
        // 150 items here  // LINT1003: Array has 150 items
    ]
}
```

**Recommended Solution**

1. Move large datasets to separate files:
```mon
import { items } from "./items_data.mon"

{
    items: *items
}
```

2. Consider pagination or chunking for very large arrays

**Configuration**

```mon
{
    max_array_items: 200  // Increase if needed
}
```

---

### LINT2001: Unused Anchor Definition

**Severity**: Warning  
**Category**: Code Smell

**Description**

An anchor is defined but never referenced with an alias (`*anchor`) or spread (`...*anchor`). This is dead code.

**Problematic Code**

```mon
{
    &base_config: { timeout: 30 },  // LINT2001: Anchor 'base_config' is defined but never used
    &used_config: { retries: 3 },
    
    settings: *used_config  // Only used_config is referenced
}
```

**Recommended Solution**

1. Remove the unused anchor:
```mon
{
    &used_config: { retries: 3 },
    settings: *used_config
}
```

2. Or use it:
```mon
{
    &base_config: { timeout: 30 },
    &used_config: { retries: 3 },
    
    default: *base_config,  // Now it's used
    settings: *used_config
}
```

**Configuration**

```mon
{
    warn_unused_anchors: false  // Disable if you want to keep unused anchors
}
```

---

### LINT2002: Duplicate Object Key

**Severity**: Error  
**Category**: Code Smell

**Description**

An object has the same key defined multiple times. The second occurrence will silently override the first, which is almost always a bug.

**Problematic Code**

```mon
{
    timeout: 30,
    retries: 3,
    timeout: 60  // LINT2002: Duplicate key 'timeout'
}
```

**Recommended Solution**

Remove the duplicate, keeping only one:
```mon
{
    timeout: 60,  // Keep the intended value
    retries: 3
}
```

**Configuration**

This rule is always enabled and cannot be disabled.

---

### LINT2003: Too Many Spread Operators

**Severity**: Warning  
**Category**: Code Smell  
**Default Limit**: 3 spreads

**Description**

Using too many spread operators in a single object makes it difficult to understand the final shape and field precedence.

**Problematic Code**

```mon
{
    config: {
        ...*base,
        ...*database,
        ...*cache,
        ...*logging  // LINT2003: Object has 4 spreads
    }
}
```

**Recommended Solution**

Consolidate spreads or restructure:
```mon
{
    &combined_base: {
        ...*base,
        ...*database
    },
    
    config: {
        ...*combined_base,
        ...*cache,
        ...*logging
    }
}
```

**Configuration**

```mon
{
    max_spreads_per_object: 5  // Increase if needed
}
```

---

### LINT2004: Magic Number Literal

**Severity**: Info  
**Category**: Code Smell

**Description**

Literal numbers without context are hard to understand. Extract them as named constants.

**Problematic Code**

```mon
{
    timeout: 3600  // LINT2004: Consider extracting 3600 into named constant
}
```

**Recommended Solution**

Use descriptive names:
```mon
{
    &one_hour: 3600,
    timeout: *one_hour  // Much clearer!
}
```

Or add a comment:
```mon
{
    timeout: 3600  // One hour in seconds
}
```

**Configuration**

```mon
{
    warn_magic_numbers: true  // Enable this check
}
```

---

### LINT3001: Missing Type Validation

**Severity**: Info  
**Category**: Best Practice

**Description**

Data lacks type validation. Adding type constraints ensures data integrity.

**Problematic Code**

```mon
{
    user: {  // LINT3001: Missing type validation
        id: 1,
        name: "Alice"
    }
}
```

**Recommended Solution**

Add type validation:
```mon
{
    User: #struct {
        id(Number),
        name(String)
    },
    
    user :: User = {
        id: 1,
        name: "Alice"
    }
}
```

**Configuration**

```mon
{
    suggest_type_validation: true  // Enable this suggestion
}
```

---

### LINT4001: Deep Import Chain

**Severity**: Warning  
**Category**: Architecture  
**Default Limit**: 2 levels

**Description**

An anchor or type is imported through multiple levels of files. This creates tight coupling.

**Problematic Code**

```
file1.mon: &base_config
file2.mon: import { &base_config } from "file1.mon", extends it
file3.mon: import { &extended } from "file2.mon"  // LINT4001: 2-level deep chain
```

**Recommended Solution**

1. Import directly from the origin:
```mon
import { &base_config } from "./file1.mon"
```

2. Consolidate related config in one file

**Configuration**

```mon
{
    max_import_chain_depth: 3  // Increase if your architecture requires it
}
```

---

### LINT4002: Circular Dependency

**Severity**: Error  
**Category**: Architecture

**Description**

Files import each other in a cycle. This can cause resolution issues.

**Problematic Code**

```
file1.mon: import { something } from "./file2.mon"
file2.mon: import { other } from "./file1.mon"  // LINT4002: Circular dependency
```

**Recommended Solution**

1. Extract shared types/anchors to a third file:
```
shared.mon: &common_config
file1.mon: import { &common_config } from "./shared.mon"
file2.mon: import { &common_config } from "./shared.mon"
```

2. Reorganize module structure to remove cycles

**Configuration**

This rule is always enabled and cannot be disabled.

---

## Summary

**Severity Levels**:
- `[E]` **Error**: Must be fixed, prevents successful linting
- `[W]` **Warning**: Should be addressed, but not blocking
- `[I]` **Info**: Suggestions for improvement

**Default Configuration**:
Most warnings are enabled with sensible defaults. Info-level suggestions are disabled by default to avoid noise.

**Best Practices**:
1. Run `mon check --lint` regularly during development
2. Configure limits based on your project's needs
3. Use `mon check --explain <CODE>` for detailed rule information
4. Keep `.monlint.mon` in version control for team consistency
