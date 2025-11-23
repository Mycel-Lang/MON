# MON Builtin Schemas

This directory contains builtin type definitions that can be imported using the `mon:` URI scheme.

## Available Schemas

### Linter Configuration

```mon
import { LintConfig } from "mon:types/linter"

config :: LintConfig = {
    max_nesting_depth: 3,
    warn_unused_anchors: true,
}
```

### Formatter Configuration

```mon
import { FormatConfig } from "mon:types/formatter"

config :: FormatConfig = {
    indent_size: 2,
    line_width: 100,
    sort_keys: "alpha",
}
```

## Using Builtin Schemas

Builtin schemas are automatically available when using the `mon:` prefix:

- `mon:types/linter` - Lint configuration types
- `mon:types/formatter` - Format configuration types

The resolver will look for these schemas in:
1. `$MON_BUILTIN_PATH` (if set)
2. `~/.mon/schemas/`
3. `/usr/share/mon/schemas/`
4. Current directory (fallback)

## Installation

For development, set `MON_BUILTIN_PATH`:

```bash
export MON_BUILTIN_PATH=/path/to/mon/schemas
```

For system-wide installation, copy to `/usr/share/mon/schemas/`.
