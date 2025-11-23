# MON CLI - Format Limitation Workarounds

This document describes the enhancements added to work around format limitations.

## TOML Null Value Handling

TOML format does not support null values natively. The MON CLI provides two strategies:

### 1. Automatic Null Removal (Default)

By default, null values are automatically removed from the output:

```bash
$ mon compile file.mon --to toml
# Null fields are omitted from output
```

**Example:**
```json
{
  "name": "Alice",
  "email": null
}
```

Becomes:
```toml
name = "Alice"
# email field is omitted
```

### 2. Null Value Replacement

Use `--toml-null-value` to replace null values with a custom string:

```bash
$ mon compile file.mon --to toml --toml-null-value "N/A"
```

**Example:**
```toml
name = "Alice"
email = "N/A"
```

## JSON Schema Generation

Generate JSON Schema from MON structures:

```bash
$ mon compile file.mon --to json-schema
```

**Example Input (structs.mon):**
```mon
{
    User: #struct {
        id(Number),
        name(String),
        email(String) = null
    },
    
    valid_user :: User = {
        id: 1,
        name: "Alice"
    }
}
```

**Output:**
```json
{
  "type": "object",
  "properties": {
    "valid_user": {
      "type": "object",
      "properties": {
        "email": {
          "type": "null"
        },
        "id": {
          "type": "number"
        },
        "name": {
          "type": "string"
        }
      },
      "required": ["id", "name"]
    }
  },
  "required": ["valid_user"]
}
```

The schema inference automatically:
- Detects primitive types (string, number, boolean, null)
- Infers object structures with properties
- Determines required vs optional fields
- Handles arrays (infers type from first element)
- Distinguishes integers from floating-point numbers

## Supported Output Formats

- `json` - JSON output (default)
- `yaml` - YAML output
- `toml` - TOML output (with null handling)
- `json-schema` - JSON Schema generation

## Examples

```bash
# Standard JSON output
mon compile config.mon

# YAML output
mon compile config.mon --to yaml

# TOML with null removal (default)
mon compile config.mon --to toml

# TOML with null replacement
mon compile config.mon --to toml --toml-null-value "undefined"

# Generate JSON Schema
mon compile types.mon --to json-schema
```
