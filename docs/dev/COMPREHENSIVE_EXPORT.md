# MON CLI - Comprehensive Export Guide

## Systems Engineer's Dream: One Command, Maximum Compatibility

The MON CLI offers a **comprehensive export mode** that generates all possible output formats in one command, organized in a clean folder structure.

## Quick Start

```bash
# Export everything to a folder
mon compile config.mon --output-dir ./output

# Result: organized folder with ALL formats
./output/
├── config.json          # JSON data
├── config.schema.json   # JSON Schema
├── config.yaml          # YAML data
├── config.toml          # TOML data
└── README.md            # Usage guide
```

## Why Use Comprehensive Export?

**Maximum Compatibility**: Generate all formats at once for different tools and platforms:

- **JSON**: Universal data interchange
- **JSON Schema**: Validation and documentation
- **YAML**: Human-readable configuration
- **TOML**: Clean config files

**Benefits**:

- One command instead of four
- Consistent naming across formats
- Automatic README generation
- Schema validation ready out-of-the-box
- No manual format conversion needed

## Examples

### Basic Export

```bash
mon compile app.mon --output-dir ./dist
```

### Export with TOML Null Handling

```bash
# Replace null values with "undefined" in TOML
mon compile config.mon --output-dir ./output --toml-null-value "undefined"
```

### CI/CD Integration

```bash
#!/bin/bash
# Build script that generates all formats for deployment

mon compile production.mon --output-dir ./build/config

# Now you have:
# - JSON for your API server
# - YAML for Kubernetes configs
# - TOML for Rust services
# - Schema for validation
```

## Generated Files

### Data Files

- `<filename>.json` - JSON data representation
- `<filename>.yaml` - YAML data representation
- `<filename>.toml` - TOML data representation (with smart null handling)

### Schema Files

- `<filename>.schema.json` - JSON Schema for validation

The schema automatically:

- Infers types from your data
- Identifies required vs optional fields
- Supports nested objects and arrays
- Works with standard validators (ajv, jsonschema, etc.)

### Documentation

- `README.md` - Auto-generated usage guide

## Single Format Output

If you only need one format, use the standard compile command:

```bash
# JSON (default)
mon compile file.mon

# Specific format
mon compile file.mon --to yaml
mon compile file.mon --to toml
mon compile file.mon --to json-schema
```

## Use Cases

### Configuration Management

```bash
# Export config for multiple environments
mon compile prod.mon --output-dir ./config/prod
mon compile dev.mon --output-dir ./config/dev
```

### API Documentation

```bash
# Generate schema for API validation
mon compile api-types.mon --output-dir ./docs/api
# Use api-types.schema.json with OpenAPI/Swagger
```

### Polyglot Projects

```bash
# One source, multiple consumers
mon compile shared-types.mon --output-dir ./shared

# JavaScript uses JSON + schema
# Python uses YAML
# Rust uses TOML
# Everyone validates against the schema
```

## Integration Examples

### JavaScript (with ajv)

```javascript
const Ajv = require("ajv");
const schema = require("./output/config.schema.json");
const data = require("./output/config.json");

const ajv = new Ajv();
const validate = ajv.compile(schema);
const valid = validate(data);
```

### Python (with jsonschema)

```python
import json
from jsonschema import validate

with open('output/config.schema.json') as f:
    schema = json.load(f)

with open('output/config.json') as f:
    data = json.load(f)

validate(instance=data, schema=schema)
```

## Best Practices

1. **Version Control**: Check in your `.mon` files, generate outputs in CI/CD
2. **Documentation**: Use the auto-generated README as a starting point
3. **Validation**: Always use the schema in production environments
4. **Null Handling**: Use `--toml-null-value` when TOML compatibility is needed

## Compatibility First

The comprehensive export mode embodies the principle of **maximum compatibility**:

- No vendor lock-in
- Works with any tool that supports standard formats
- Schema validation follows JSON Schema spec
- Clean, predictable file structure

**One command. All formats. Maximum compatibility. 🎉**
