# Import Analysis (LINT4XXX)

Issues with imports and module dependencies.

## LINT4001: Deep Import Chain

**Severity**: Warning  
**Category**: Import Analysis

### What It Does

Warns when a file has excessive namespaced references, indicating complex import chains.

### Problem

```mon
///  file.mon
import * as utils from "./utils.mon"
import * as helpers from "./helpers.mon"
import * as config from "./config.mon"

{
    data: *utils.parser.data,      // Deeply nested namespace
    result: *helpers.formatter.process,
    setting: *config.app.database.host,
    // ... many more namespaced references
}
```

### Why It Matters

- **Complexity**: Hard to track where values come from
- **Coupling**: Too many dependencies
- **Performance**: More files to parse and resolve
- **Maintenance**: Changes ripple through many files

### Solution

#### Option 1: Import Specific Items

```mon
// ✅ Better: Import only what you need
import { parser, formatter } from "./utils.mon"
import { database_config } from "./config.mon"

{
    data: *parser.data,
    result: *formatter.process,
    setting: *database_config.host
}
```

#### Option 2: Flatten Structure

```mon
// ✅ Best: Simplify your module structure
import { data, process, database_host } from "./all.mon"

{
    data: *data,
    result: *process,
    setting: *database_host
}
```

### Configuration

```mon
// .monlint.mon
{
    max_import_chain_depth: 5,  // Allow deeper chains (default: 3)
}
```

---

## LINT4002: Circular Dependency

**Severity**: Error  
**Category**: Import Analysis

### What It Does

Detects circular imports between files (A imports B, B imports A).

### Problem

```mon
// user.mon
import { Profile } from "./profile.mon"

{
    User: #struct {
        profile(Profile)
    }
}
```

```mon
// profile.mon
import {  User } from "./user.mon"   // ❌ Circular!

{
    Profile: #struct {
        user(User)
    }
}
```

### Why It Matters

- **Resolution Error**: Cannot resolve which file to load first
- **Maintenance Nightmare**: Changes affect both files
- **Design Smell**: Usually indicates poor module structure

### Solution

#### Option 1: Merge Files

```mon
// user-profile.mon
{
    User: #struct {
        profile(Profile)
    },
    
    Profile: #struct {
        bio(String),
        avatar(String)
    }
}
```

#### Option 2: Extract Shared Types

```mon
// types.mon
{
    UserId: #struct { id(Number) },
    ProfileData: #struct { bio(String) }
}
```

```mon
// user.mon
import { ProfileData } from "./types.mon"

{
    User: #struct {
        profile(ProfileData)
    }
}
```

```mon
// profile.mon
import { UserId } from "./types.mon"

{
    Profile: #struct {
        user_id(UserId),
        bio(String)
    }
}
```

### Configuration

This rule cannot be disabled - circular dependencies are always errors.

---

## LINT4003: Unused Import

**Severity**: Warning  
**Category**: Import Analysis

### What It Does

Detects imported items that are never used in the file.

### Problem

```mon
import { User, Profile, Settings } from "./types.mon"

{
    user :: User = {              // ✅ User is used
        name: "Alice"
    }
    // ⚠️ Warning: Profile and Settings are never used
}
```

### Why It Matters

- **Clutter**: Makes code harder to understand
- **Performance**: Unnecessary parsing
- **Confusion**: Readers wonder why it's imported

### Solution

```mon
import { User } from "./types.mon"    // ✅ Import only what you use

{
    user :: User = {
        name: "Alice"
    }
}
```

### Configuration

```mon
// .monlint.mon
{
    // Note: This rule is detected by the resolver
    // No specific configuration option yet
}
```

---

## Best Practices

### Keep Imports Clean

❌ **Bad:**
```mon
import * as everything from "./everything.mon"
import { A, B, C, D, E, F, G } from "./alphabet.mon"
```

✅ **Good:**
```mon
import { User, Config } from "./types.mon"
import { validate } from "./utils.mon"
```

### Organize Imports

```mon
// 1. Builtin schemas first
import { LintConfig } from "mon:types/linter"

// 2. External/shared modules
import { DatabaseConfig } from "./shared/config.mon"

// 3. Local modules
import { &app_defaults } from "./defaults.mon"

{
    // Your config here
}
```

### Avoid Deep Nesting

❌ **Bad:**
```mon
import * as app from "./deeply/nested/structure/app.mon"

value: *app.config.database.production.primary.host
```

✅ **Good:**
```mon
import { database_host } from "./config.mon"

value: *database_host
```

---

## Summary

| Code | Rule | Severity | Configurable |
|------|------|----------|--------------|
| LINT4001 | DeepImportChain | Warning | Yes (`max_import_chain_depth`) |
| LINT4002 | CircularDependency | Error | No (always enforced) |
| LINT4003 | UnusedImport | Warning | Not yet |

## Quick Config

```mon
// .monlint.mon
{
    max_import_chain_depth: 5,  // Allow deeper import chains
}
```

## See Also

- [Complexity Rules](complexity.md) - LINT1XXX
- [Code Smells](smells.md) - LINT2XXX
- [Best Practices](best-practices.md) - LINT3XXX
