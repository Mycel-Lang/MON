# API Overview - Using MON as a Library

MON can be used as a Rust library for programmatic analysis and manipulation of MON files.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mon = "0.1"
mon-core = "0.1"  # Core parser and AST
```

## Quick Start

```rust
use mon::linter::{AnalysisService, LintConfig};

fn main() -> miette::Result<()> {
    let source = r#"{
        timeout: 3600,
        retries: 5
    }"#;
    
    // Analyze the document
    let service = AnalysisService::new(LintConfig::default());
    let result = service.analyze_document(source, "config.mon")?;
    
    // Check for diagnostics
    for diagnostic in &result.diagnostics {
        println!("{}: {}", diagnostic.code, diagnostic.message);
    }
    
    // Use symbol table
    println!("Found {} symbols", result.symbol_table.symbol_count());
    
    Ok(())
}
```

## Core Modules

### `mon::linter`

Code quality analysis and linting.

```rust
use mon::linter::{
    AnalysisService,      // Main API
    LintConfig,           // Configuration
    SymbolTable,          // Symbol tracking
    DiagnosticCode,       // Lint rule codes
};
```

**See**: [Analysis Service](analysis-service.md)

### `mon-core::parser`

Parsing MON source to AST.

```rust
use mon_core::parser::Parser;
use mon_core::ast::{MonDocument, MonValue, MonValueKind};

let mut parser = Parser::new(source)?;
let doc = parser.parse_document()?;
```

**See**: [mon-core documentation](https://docs.rs/mon-core)

### `mon-core::api`

High-level operations (compile, resolve).

```rust
use mon_core::api::{analyze, compile_to_json};

// Full analysis with resolution
let result = analyze(source, "file.mon")?;

// Simple compilation
let json = compile_to_json(source)?;
```

## Common Use Cases

### 1. Lint a File

```rust
use mon::linter::{AnalysisService, LintConfig};
use std::fs;

fn lint_file(path: &str) -> miette::Result<()> {
    let source = fs::read_to_string(path)?;
    
    let service = AnalysisService::new(LintConfig::default());
    let result = service.analyze_document(&source, path)?;
    
    if result.has_errors() {
        eprintln!("Found {} errors", result.errors().len());
        for error in result.errors() {
            eprintln!("  {}: {}", error.code, error.message);
        }
        std::process::exit(1);
    }
    
    Ok(())
}
```

### 2. Parse and Extract Data

```rust
use mon_core::parser::Parser;
use mon_core::ast::{MonValueKind, Member};

fn extract_config(source: &str) -> miette::Result<()> {
    let mut parser = Parser::new(source)?;
    let doc = parser.parse_document()?;
    
    if let MonValueKind::Object(members) = &doc.root.kind {
        for member in members {
            if let Member::Pair(pair) = member {
                println!("{} = {:?}", pair.key, pair.value.kind);
            }
        }
    }
    
    Ok(())
}
```

### 3. Find All Anchors

```rust
use mon::linter::{AnalysisService, LintConfig, SymbolKind};

fn find_anchors(source: &str) -> miette::Result<Vec<String>> {
    let service = AnalysisService::new(LintConfig::default());
    let result = service.analyze_document(source, "input.mon")?;
    
    let anchors: Vec<String> = result
        .symbol_table
        .symbols_by_kind(SymbolKind::Anchor)
        .iter()
        .map(|sym| sym.name.clone())
        .collect();
    
    Ok(anchors)
}
```

### 4. Track Symbol References

```rust
use mon::linter::{AnalysisService, LintConfig, SymbolKind};

fn find_usages(source: &str, anchor_name: &str) -> miette::Result<usize> {
    let service = AnalysisService::new(LintConfig::default());
    let result = service.analyze_document(source, "input.mon")?;
    
    let refs = result
        .symbol_table
        .find_references(anchor_name, SymbolKind::Anchor);
    
    println!("Anchor '{}' is used {} times", anchor_name, refs.len());
    
    for ref_info in refs {
        println!("  at line {}", ref_info.range.start.line);
    }
    
    Ok(refs.len())
}
```

### 5. Convert to JSON

```rust
use mon_core::api::analyze;
use serde_json::Value;

fn mon_to_json(source: &str) -> miette::Result<Value> {
    let result = analyze(source, "input.mon")?;
    let json_str = result.to_json()?;
    let value: Value = serde_json::from_str(&json_str)?;
    Ok(value)
}
```

### 6. Custom Linter Configuration

```rust
use mon::linter::{AnalysisService, LintConfig};

fn lint_strict(source: &str) -> miette::Result<()> {
    let config = LintConfig {
        max_nesting_depth: 3,
        max_object_members: 10,
        max_array_items: 20,
        warn_unused_anchors: true,
        warn_magic_numbers: true,
        suggest_type_validation: true,
        ..Default::default()
    };
    
    let service = AnalysisService::new(config);
    let result = service.analyze_document(source, "input.mon")?;
    
    for diagnostic in &result.diagnostics {
        println!("[{}] {}", diagnostic.severity, diagnostic.message);
    }
    
    Ok(())
}
```

## Error Handling

All MON operations return `miette::Result<T>` for rich error reporting:

```rust
use miette::{Result, IntoDiagnostic};

fn process_file(path: &str) -> Result<()> {
    let source = std::fs::read_to_string(path)
        .into_diagnostic()?;
    
    let service = AnalysisService::new(LintConfig::default());
    let result = service.analyze_document(&source, path)?;
    
    // Errors have source locations and context
    if result.has_errors() {
        return Err(miette::miette!("Linting failed"));
    }
    
    Ok(())
}
```

## Performance Tips

1. **Reuse AnalysisService** - Create once, analyze multiple files
2. **Parse once** - Don't re-parse if you just need different views
3. **Async for I/O** - Use `tokio::fs` for file operations
4. **Cache results** - Symbol tables can be cached between runs

## Integration Examples

### Build Script

```rust
// build.rs
use mon_core::api::analyze;
use std::env;

fn main() {
    let config_path = concat!(env!("CARGO_MANIFEST_DIR"), "/config.mon");
    let source = std::fs::read_to_string(config_path).unwrap();
    
    let result = analyze(&source, config_path).unwrap();
    let json = result.to_json().unwrap();
    
    // Write to OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    std::fs::write(
        format!("{}/config.json", out_dir),
        json
    ).unwrap();
}
```

### Macro for Compile-Time MON

```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn mon(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    
    // Parse and validate at compile time
    let mut parser = Parser::new(&source).expect("Parse error");
    let doc = parser.parse_document().expect("Invalid MON");
    
    // Generate Rust code from MON
    // ...
}
```

## See Also

- [Analysis Service API](analysis-service.md) - Detailed API reference
- [Symbol Table API](symbol-table.md) - Symbol tracking
- [Diagnostics](diagnostics.md) - Working with lint results
- [mon-core docs](https://docs.rs/mon-core) - Core library reference
