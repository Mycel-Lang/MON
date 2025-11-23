# CLI Workflows

Common workflows and real-world usage patterns for MON CLI tools.

## Development Workflow

### 1. Write MON Configuration

Create `config.mon`:

```mon
{
    database: {
        host: "localhost",
        port: 5432,
        pool_size: 10
    },
    
    cache: {
        ttl: 3600,
        max_entries: 1000
    }
}
```

### 2. Format While Editing

```bash
# Format and check
mon fmt config.mon

# Or auto-format on save (watch mode)
mon fmt . --watch
```

### 3. Validate and Lint

```bash
# Check syntax
mon check config.mon

# Run linter
mon check config.mon --lint

# Get JSON for tooling
mon check config.mon --lint --as-json > lint.json
```

### 4. Compile for Deployment

```bash
# Convert to JSON for app
mon compile config.mon --to json > config.json

# Or YAML for K8s
mon compile config.mon --to yaml > config.yaml
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: MON CI
on: [push, pull_request]

jobs:
  check-mon:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install MON
        run: cargo install mon
      
      - name: Check Formatting
        run: mon fmt **/*.mon --check
      
      - name: Lint
        run: |
          EXIT=0
          for file in $(find . -name "*.mon"); do
            mon check "$file" --lint || EXIT=1
          done
          exit $EXIT
      
      - name: Compile Configs
        run: |
          mkdir -p dist
          for file in configs/*.mon; do
            base=$(basename "$file" .mon)
            mon compile "$file" --to json -o "dist/$base.json"
          done
      
      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: compiled-configs
          path: dist/
```

### GitLab CI

```yaml
lint-mon:
  stage: test
  script:
    - cargo install mon
    - mon fmt **/*.mon --check
    - find . -name "*.mon" -exec mon check {} --lint \;
  
compile-mon:
  stage: build
  script:
    - cargo install mon
    - mkdir dist
    - |
      for file in configs/*.mon; do
        mon compile "$file" --to json -o "dist/$(basename $file .mon).json"
      done
  artifacts:
    paths:
      - dist/
```

---

## Team Collaboration

### Setup Team Standards

```bash
# Create configs
cat > .monfmt.mon <<EOF
{
    indent_size: 4,
    max_line_width: 100,
    trailing_commas: "always",
    sort_keys: "alpha"
}
EOF

cat > .monlint.mon <<EOF
{
    max_nesting_depth: 5,
    warn_unused_anchors: true,
    warn_magic_numbers: true
}
EOF

# Commit to repo
git add .monfmt.mon .monlint.mon
git commit -m "Add MON standards"
```

### Pre-commit Hook

```bash
# .git/hooks/pre-commit
#!/bin/bash

echo "Running MON checks..."

# Format staged MON files
FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.mon$')

if [ -n "$FILES" ]; then
    for file in $FILES; do
        # Format
        mon fmt "$file" --write --config .monfmt.mon
        
        # Lint
        mon check "$file" --lint || {
            echo "❌ Lint failed for $file"
            exit 1
        }
        
        # Re-stage
        git add "$file"
    done
    
    echo "✅ All MON files formatted and linted"
fi
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

---

## Multi-Environment Setup

### Directory Structure

```
configs/
├── base.mon          # Shared config
├── dev.mon           # Development
├── staging.mon       # Staging
└── production.mon    # Production
```

### Base Configuration

**`configs/base.mon`**:
```mon
{
    &base_db: {
        pool_size: 10,
        timeout: 5000
    },
    
    &base_cache: {
        ttl: 3600
    }
}
```

### Environment-Specific

**`configs/dev.mon`**:
```mon
import { &base_db, &base_cache } from "./base.mon"

{
    environment: "development",
    
    database: {
        ...*base_db,
        host: "localhost",
        port: 5432
    },
    
    cache: {
        ...*base_cache,
        host: "localhost"
    }
}
```

**`configs/production.mon`**:
```mon
import { &base_db, &base_cache } from "./base.mon"

{
    environment: "production",
    
    database: {
        ...*base_db,
        host: "prod-db.example.com",
        port: 5432,
        pool_size: 50,  // Override
        ssl: true
    },
    
    cache: {
        ...*base_cache,
        host: "prod-redis.example.com",
        ttl: 7200  // Override
    }
}
```

### Build Script

```bash
#!/bin/bash
# scripts/build-configs.sh

ENVS="dev staging production"
OUT_DIR="dist/configs"

mkdir -p "$OUT_DIR"

for env in $ENVS; do
    echo "Building $env config..."
    
    mon check "configs/$env.mon" --lint || exit 1
    
    mon compile "configs/$env.mon" \
        --to json \
        --output-dir "$OUT_DIR/$env" \
        --generate-docs
    
    echo "✅ Built $env config"
done

echo "✨ All configs built successfully"
```

---

## Monorepo Management

### Structure

```
monorepo/
├── .monfmt.mon       # Root formatter config
├── .monlint.mon      # Root linter config
├── services/
│   ├── api/
│   │   ├── config.mon
│   │   └── schema.mon
│   └── worker/
│       └── config.mon
└── shared/
    └── types.mon
```

### Bulk Operations

```bash
# Format all MON files
find . -name "*.mon" -exec mon fmt {} --write \;

# Lint all services
for service in services/*; do
    echo "Linting $service..."
    find "$service" -name "*.mon" -exec mon check {} --lint \;
done

# Compile all configs
find services -name "config.mon" -exec sh -c '
    dir=$(dirname "$1")
    mon compile "$1" --to json > "$dir/config.json"
' _ {} \;
```

---

## Migration from JSON/YAML

### Convert Existing Files

```bash
# Assuming you have JSON configs
for json_file in configs/*.json; do
    base=$(basename "$json_file" .json)
    
    # Manual conversion needed (no auto-convert yet)
    # Create MON version
    cat > "configs/$base.mon" <<EOF
{
    // Converted from $json_file
    // TODO: Add types and anchors
}
EOF
    
    echo "Created configs/$base.mon - manual editing required"
done
```

### Gradual Migration

1. **Keep both formats temporarily**
2. **Build from MON** → JSON/YAML for deployment
3. **Remove old files** once confident

```bash
# Build both for comparison
mon compile config.mon --to json > config.json.new
diff config.json config.json.new
```

---

## Troubleshooting

### Parse Errors

```bash
# Detailed error with context
mon check broken.mon

# Check specific line
mon check broken.mon | grep "line 10"
```

### Lint Issues

```bash
# Get specific rule info
mon check --explain LINT2001

# Disable rule temporarily
# Add to .monlint.mon:
# disabled_rules: ["LINT2001"]
```

### Format Differences

```bash
# See what would change
mon fmt file.mon

# Apply if looks good
mon fmt file.mon --write
```

## See Also

- [Check Command](../cli/check.md)
- [Format Command](../cli/fmt.md)
- [Example Configs](configs.md)
- [API Integration](integration.md)
