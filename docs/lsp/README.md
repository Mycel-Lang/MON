# LSP Infrastructure Overview

The MON linter includes infrastructure ready for Language Server Protocol (LSP) integration.

## Architecture

```
┌──────────────────────────────────────┐
│       AnalysisService (API)          │
│  - Parse and analyze documents       │
│  - Build symbol table                │
│  - Generate diagnostics              │
└───────────┬──────────────────────────┘
            │
    ┌───────┴────────┬──────────────┬──────────┐
    │                │              │          │
┌───▼───┐      ┌────▼────┐    ┌───▼───┐  ┌──▼───────┐
│Parser │      │ Symbol  │    │ Lint  │  │ Position │
│(Core) │      │  Table  │    │ Rules │  │ System   │
└───────┘      └─────────┘    └───────┘  └──────────┘
```

## Components

### 1. Position System

LSP-compatible position tracking:

```rust
use mon::linter::{Position, Range, Location};

// Zero-based positions (LSP standard)
let pos = Position { line: 0, character: 5 };

// Ranges (inclusive start, exclusive end)
let range = Range {
    start: Position { line: 0, character: 0 },
    end: Position { line: 0, character: 10 },
};

// Locations (file + range)
let location = Location {
    uri: "file:///path/to/file.mon".to_string(),
    range,
};
```

**Features**:
- Byte offset → Position conversion
- UTF-16 character offsets (LSP requirement)
- Range containment checking

**See**: [Position System](positions.md)

### 2. Symbol Table

Tracks all definitions and references:

```rust
use mon::linter::{SymbolTable, Symbol, SymbolKind, SymbolReference};

let mut table = SymbolTable::new();

// Add a symbol definition
table.add_symbol(Symbol {
    name: "my_anchor".to_string(),
    kind: SymbolKind::Anchor,
    range: Range { /* ... */ },
    detail: Some("{ value: 1 }".to_string()),
    documentation: None,
});

// Add a reference
table.add_reference(SymbolReference {
    symbol_name: "my_anchor".to_string(),
    symbol_kind: SymbolKind::Anchor,
    range: Range { /* ... */ },
    reference_kind: ReferenceKind::Alias,
});

// Find definition
let symbol = table.find_symbol("my_anchor", SymbolKind::Anchor);

// Find all references
let refs = table.find_references("my_anchor", SymbolKind::Anchor);
```

**Features**:
- Track anchors, types, imports
- Find definitions
- Find all references
- Detect unused symbols

**See**: [Symbol Table](symbol-table.md)

### 3. Diagnostics

LSP-compatible diagnostic messages:

```rust
use mon::linter::{Diagnostic, DiagnosticCode, DiagnosticSeverity};

let diagnostic = Diagnostic {
    code: DiagnosticCode::UnusedAnchor,
    code_name: "UnusedAnchor".to_string(),
    severity: DiagnosticSeverity::Warning,
    message: "Anchor 'foo' is defined but never used".to_string(),
    range: Some(Range { /* ... */ }),
    related_information: vec![],
    tags: vec![DiagnosticTag::Unnecessary],
    location: None,
};
```

**JSON Output**:
```json
{
  "code": "LINT2001",
  "code_name": "UnusedAnchor",
  "severity": "Warning",
  "message": "Anchor 'foo' is defined but never used",
  "range": {
    "start": { "line": 2, "character": 4 },
    "end": { "line": 2, "character": 7 }
  },
  "tags": ["Unnecessary"]
}
```

## LSP Features Ready

### ✅ Already Implemented

1. **Text Document Diagnostics**
   - Precise ranges
   - Severity levels
   - Diagnostic tags
   - Related information

2. **Symbol Tracking**
   - Definition locations
   - Reference locations
   - Symbol kinds
   - Unused detection

3. **Position Mapping**
   - Byte offsets → Line/column
   - UTF-16 compatibility
   - Range operations

### 🔜 Easy to Add (Future `mon-lsp`)

1. **Go to Definition**
   ```rust
   fn goto_definition(position: Position) -> Option<Location> {
       result.symbol_table.find_symbol_at(position)
           .map(|sym| Location {
               uri: current_file_uri(),
               range: sym.range,
           })
   }
   ```

2. **Find All References**
   ```rust
   fn find_references(position: Position) -> Vec<Location> {
       let symbol = result.symbol_table.find_symbol_at(position)?;
       result.symbol_table
           .find_references(&symbol.name, symbol.kind)
           .iter()
           .map(|r| Location {
               uri: current_file_uri(),
               range: r.range,
           })
           .collect()
   }
   ```

3. **Hover Information**
   ```rust
   fn hover(position: Position) -> Option<Hover> {
       result.symbol_table.find_symbol_at(position)
           .map(|sym| Hover {
               contents: format!("{}\n\n{}", 
                   sym.detail.unwrap_or_default(),
                   sym.documentation.unwrap_or_default()
               ),
               range: Some(sym.range),
           })
   }
   ```

4. **Document Symbols (Outline)**
   ```rust
   fn document_symbols() -> Vec<DocumentSymbol> {
       result.symbol_table
           .symbols_by_kind(SymbolKind::Anchor)
           .iter()
           .map(|sym| DocumentSymbol {
               name: sym.name.clone(),
               kind: SymbolKind::Variable,
               range: sym.range,
               selection_range: sym.range,
           })
           .collect()
   }
   ```

## Creating `mon-lsp` (Future)

### 1. Project Structure

```
mon-lsp/
├── Cargo.toml
└── src/
    ├── main.rs           # LSP server entry
    ├── server.rs         # Backend implementation
    ├── handlers/
    │   ├── diagnostics.rs
    │   ├── definition.rs
    │   ├── references.rs
    │   └── hover.rs
    └── convert.rs        # mon types → LSP types
```

### 2. Dependencies

```toml
[dependencies]
mon = { path = "../mon" }
tower-lsp = "0.20"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

### 3. Server Implementation

```rust
use tower_lsp::*;
use mon::linter::AnalysisService;

struct MonLspBackend {
    service: AnalysisService,
    documents: HashMap<Url, AnalysisResult>,
}

#[tower_lsp::async_trait]
impl LanguageServer for MonLspBackend {
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        // Analyze document
        let result = self.service
            .analyze_document(&text, uri.path())
            .unwrap();
        
        // Store result
        self.documents.insert(uri.clone(), result);
        
        // Publish diagnostics
        self.publish_diagnostics(uri, result.diagnostics).await;
    }
    
    async fn goto_definition(&self, params: GotoDefinitionParams) 
        -> Option<GotoDefinitionResponse> 
    {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        let result = self.documents.get(&uri)?;
        let symbol = result.symbol_table.find_symbol_at(position)?;
        
        Some(GotoDefinitionResponse::Scalar(Location {
            uri,
            range: symbol.range,
        }))
    }
}
```

## Performance Considerations

### Incremental Analysis

```rust
// Cache results per file
struct DocumentCache {
    source: String,
    result: AnalysisResult,
    version: i32,
}

// Only re-analyze on change
fn on_change(&mut self, uri: Url, text: String, version: i32) {
    if let Some(cached) = self.cache.get(&uri) {
        if cached.version == version {
            return; // No change
        }
    }
    
    // Analyze and update cache
    let result = self.service.analyze_document(&text, uri.path())?;
    self.cache.insert(uri, DocumentCache { source: text, result, version });
}
```

### Debouncing

```rust
// Don't analyze on every keystroke
use tokio::time::{sleep, Duration};

async fn debounced_analysis(&self, uri: Url, text: String) {
    sleep(Duration::from_millis(300)).await;
    
    // Check if text changed during sleep
    if self.get_current_text(&uri) == text {
        self.analyze(uri, text).await;
    }
}
```

## Testing LSP Features

```rust
#[test]
fn test_goto_definition() {
    let source = r#"{
        &anchor: { value: 1 },
        ref: *anchor
    }"#;
    
    let service = AnalysisService::new(LintConfig::default());
    let result = service.analyze_document(source, "test.mon").unwrap();
    
    // Simulate cursor at position of "*anchor"
    let position = Position { line: 2, character: 14 };
    
    // Should find anchor definition
    let symbol = result.symbol_table.find_symbol("anchor", SymbolKind::Anchor);
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap().range.start.line, 1);
}
```

## See Also

- [Position System](positions.md) - Detailed position types
- [Symbol Table](symbol-table.md) - Symbol tracking API
- [Architecture](architecture.md) - System design
- [tower-lsp](https://docs.rs/tower-lsp) - LSP framework
- [LSP Specification](https://microsoft.github.io/language-server-protocol/) - Official spec
