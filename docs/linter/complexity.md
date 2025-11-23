# Linter Rule Details - Complexity (LINT1XXX)

Rules that detect overly complex code structures.

## LINT1001: MaxNestingDepth

**Severity**: Warning  
**Category**: Complexity

### Description

Detects deeply nested structures that reduce readability and maintainability. Deep nesting makes code harder to understand and test.

### Problematic Code

```mon
{
    level1: {
        level2: {
            level3: {
                level4: {
                    level5: {
                        level6: {
                            value: "too deep"  // LINT1001: Nesting depth 7 exceeds limit of 5
                        }
                    }
                }
            }
        }
    }
}
```

### Recommended Solution

**Flatten using anchors and spreads**:

```mon
{
    &level4_5_6: {
        level5: {
            level6: {
                value: "acceptable"
            }
        }
    },
    
    level1: {
        level2: {
            level3: {
                level4: *level4_5_6
            }
        }
    }
}
```

### Configuration

```mon
{
    max_nesting_depth: 5  // Default: 5, range: 3-10
}
```

---

## LINT1002: MaxObjectMembers

**Severity**: Warning  
**Category**: Complexity

### Description

Objects with too many members become difficult to manage and understand. Consider splitting into smaller, focused objects.

### Problematic Code

```mon
{
    config: {
        setting1: "value1",
        setting2: "value2",
        setting3: "value3",
        // ... 20 more settings
        setting25: "value25"  // LINT1002: Object has 25 members, limit is 20
    }
}
```

### Recommended Solution

**Split into logical groups**:

```mon
{
    &database_config: {
        host: "localhost",
        port: 5432,
        database: "myapp",
        pool_size: 10
    },
    
    &cache_config: {
        ttl: 3600,
        max_size: 1000,
        strategy: "lru"
    },
    
    config: {
        database: *database_config,
        cache: *cache_config,
        // Other top-level settings
    }
}
```

### Configuration

```mon
{
    max_object_members: 20  // Default: 20, range: 5-100
}
```

---

## LINT1003: MaxArrayItems

**Severity**: Warning  
**Category**: Complexity

### Description

Arrays with too many items may indicate a need for data restructuring or external data files.

### Problematic Code

```mon
{
    ids: [1, 2, 3, 4, 5, /* ... 50 more items ... */ 55]  
    // LINT1003: Array has 55 items, limit is 50
}
```

### Recommended Solution

**Use external data or restructure**:

```mon
{
    // Option 1: Import from separate file
    // import { &id_list } from "./data/ids.mon"
    
    // Option 2: Restructure as object
    id_ranges: {
        start: 1,
        end: 55,
        step: 1
    }
}
```

### Configuration

```mon
{
    max_array_items: 50  // Default: 50, range: 10-500
}
```

---

## Best Practices

1. **Use anchors and spreads** - Flatten deep nesting
2. **Split large objects** - Create logical groupings
3. **External data files** - For large datasets
4. **Type definitions** - Structure complex data

## See Also

- [Code Smells](smells.md) - LINT2XXX rules
- [Configuration](configuration.md) - Customize limits
- [Best Practices](best-practices.md) - General guidelines
