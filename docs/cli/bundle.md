# `mon bundle` - Import Bundler

> Resolve imports and create a single bundled file

## Synopsis

```bash
mon bundle [OPTIONS] <ENTRY_FILE>
```

## Description

The `mon bundle` command resolves all import statements in a MON project and creates a single, self-contained output file. It analyzes the import dependency graph, detects circular dependencies, and optionally performs tree-shaking to remove unused definitions.

**Use cases:**

- Deploy configuration without complex import chains
- Create standalone distribution files
- Convert MON projects to other formats (JSON, YAML)
- Analyze import dependencies

## Arguments

### `<ENTRY_FILE>`

The entry point MON file containing imports to resolve.

```bash
mon bundle main.mon
```

## Options

### `-o, --output <PATH>`

Output file path. If not specified, writes to stdout.

```bash
mon bundle main.mon -o bundled.mon
```

### `-t, --to <FORMAT>`

Output format: `mon` (default), `json`, or `yaml`.

```bash
# Bundle as MON
mon bundle main.mon -o output.mon

# Export as JSON
mon bundle main.mon --to json -o data.json

# Export as YAML
mon bundle main.mon --to yaml -o config.yaml
```

### `-m, --minify`

Minify output (remove comments and extra whitespace).

> **Note**: Full minification is not yet implemented. Currently uses default formatting.

```bash
mon bundle main.mon -m -o min.mon
```

### `--tree-shake`

Remove unused type definitions and anchors from the bundle.

> **Note**: Tree-shaking is currently a stub and will be fully implemented in a future release.

```bash
mon bundle main.mon --tree-shake -o optimized.mon
```

## Exit Codes

| Code | Meaning                                           |
| ---- | ------------------------------------------------- |
| `0`  | Bundle created successfully                       |
| `1`  | Error (file not found, circular dependency, etc.) |

## Output

```
Bundling  main.mon
  → Checking imports...
  ✓ 3 import(s) verified
  ✓ Wrote bundle to bundled.mon
```

## Circular Dependency Detection

If a circular import is detected, bundling fails with a clear error:

```bash
$ mon bundle cyclic.mon
Error: Circular dependency detected: main.mon → utils.mon → helpers.mon → main.mon
```

## Examples

### Basic Bundling

```bash
# Bundle to stdout
mon bundle entry.mon

# Bundle to file
mon bundle entry.mon -o bundle.mon
```

### Export to Different Formats

```bash
# Convert MON project to JSON
mon bundle main.mon --to json -o config.json

# Convert to YAML for consumption by other tools
mon bundle settings.mon --to yaml -o settings.yaml
```

### Optimized Build

```bash
# Create minified, tree-shaken production bundle
mon bundle app.mon -m --tree-shake -o dist/app.mon
```

### Analyze Import Graph

```bash
# See which files are imported (stderr output)
mon bundle main.mon 2>&1 | grep "import(s)"
```

## Import Resolution

The bundler resolves imports in the following order:

1. Parse entry file
2. Build import dependency graph
3. Detect circular dependencies (fail if found)
4. Recursively resolve all imports
5. Optionally tree-shake unused definitions
6. Format output in requested format

### Supported Import Patterns

```mon
# Relative imports
import { config } from "./config.mon"
import { utils } from "../shared/utils.mon"

# Named imports
import { database, cache } from "./services.mon"

# Namespace imports
import * as helpers from "./helpers.mon"
```

## Output Formats

### MON

Default output format. Creates a single MON file with all imports inlined.

> **Limitation**: Import statement formatting is simplified in v0.0.1. Full import preservation coming in v0.1.0.

### JSON

Converts the resolved MON document to JSON:

```json
{
  "database": {
    "host": "localhost",
    "port": 5432
  },
  "cache": {
    "ttl": 3600
  }
}
```

### YAML

Converts the resolved MON document to YAML:

```yaml
database:
  host: localhost
  port: 5432
cache:
  ttl: 3600
```

## Known Limitations

- Tree-shaking not yet implemented (v0.0.1)
- Minification uses default formatter (v0.0.1)
- Import statements are simplified in MON output (v0.0.1)
- No support for remote/HTTP imports

## See Also

- [Import Documentation](../../MON_LANGUAGE_REFERENCE.md#imports) (needs cleanup)
- [`mon compile`](./compile.md) - Convert single files to other formats
- [`mon check`](./check.md) - Validate syntax including import statements
