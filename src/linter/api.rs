//! Analysis service providing a clean API for linting with symbol tracking.
//!
//! This module provides the main `AnalysisService` that coordinates between
//! the linter, symbol table, and resolver to provide comprehensive analysis
//! suitable for both CLI and LSP use.

use crate::linter::{
    LintConfig, LintResult, Linter, Range, ReferenceKind, Symbol, SymbolKind,
    SymbolReference, SymbolTable,
};
use miette::Result;
use mon_core::ast::{Member, MonDocument, MonValue, MonValueKind};

/// Analysis service for comprehensive MON document analysis.
///
/// This service provides a high-level API that combines linting, symbol tracking,
/// and semantic analysis. It's designed to be used by both the CLI (`mon check --lint`)
/// and the future LSP server (`mon-lsp`).
///
/// # Examples
/// ```ignore
/// let service = AnalysisService::new(LintConfig::default());
/// let result = service.analyze_document(source, "file.mon").unwrap();
///
/// // Get diagnostics
/// for diagnostic in &result.diagnostics {
///     println!("{}: {}", diagnostic.code, diagnostic.message);
/// }
///
/// // Get symbol table for LSP features
/// let symbols = result.symbol_table.symbols_by_kind(SymbolKind::Anchor);
/// ```
pub struct AnalysisService {
    config: LintConfig,
}

impl AnalysisService {
    /// Creates a new analysis service with the given configuration.
    pub fn new(config: LintConfig) -> Self {
        Self { config }
    }

    /// Analyzes a MON document from source text.
    ///
    /// This performs:
    /// - Full linting (complexity, smells, best practices)
    /// - Symbol table construction
    /// - Unused symbol detection
    ///
    /// # Arguments
    /// * `source` - The MON source code
    /// * `file_path` - Path to the file (for diagnostics)
    ///
    /// # Returns
    /// An `AnalysisResult` containing diagnostics and symbol table
    pub fn analyze_document(&self, source: &str, file_path: &str) -> Result<AnalysisResult> {
        // Parse the document
        let mut parser = mon_core::parser::Parser::new_with_name(source, file_path.to_string())?;
        let doc = parser.parse_document()?;

        // Run standard linting
        let linter = Linter::new(self.config.clone());
        let mut lint_result = linter.lint(&doc, source)?;

        // Build symbol table
        let mut symbol_table = SymbolTable::new();
        self.build_symbol_table(&doc, source, &mut symbol_table);

        // Detect unused symbols and add diagnostics
        self.detect_unused_symbols(&symbol_table, source, &mut lint_result);

        Ok(AnalysisResult {
            diagnostics: lint_result.diagnostics,
            symbol_table,
            source: source.to_string(),
        })
    }

    /// Builds the symbol table from the AST.
    fn build_symbol_table(&self, doc: &MonDocument, source: &str, table: &mut SymbolTable) {
        // Track imports
        for import in &doc.imports {
            table.add_symbol(Symbol {
                name: import.path.clone(),
                kind: SymbolKind::Import,
                range: Range::from_byte_offsets(source, import.pos_start, import.pos_end),
                detail: Some(format!("import from {}", import.path)),
                documentation: None,
            });
        }

        // Recursively track anchors and types
        self.build_symbols_from_value(&doc.root, source, table);
    }

    /// Recursively builds symbols from a value.
    fn build_symbols_from_value(&self, value: &MonValue, source: &str, table: &mut SymbolTable) {
        // Track anchor definitions
        if let Some(anchor_name) = &value.anchor {
            table.add_symbol(Symbol {
                name: anchor_name.clone(),
                kind: SymbolKind::Anchor,
                range: Range::from_byte_offsets(source, value.pos_start, value.pos_end),
                detail: self.get_value_preview(value),
                documentation: None,
            });
        }

        // Track references (aliases and spreads)
        match &value.kind {
            MonValueKind::Alias(name) => {
                table.add_reference(SymbolReference {
                    symbol_name: name.clone(),
                    symbol_kind: SymbolKind::Anchor,
                    range: Range::from_byte_offsets(source, value.pos_start, value.pos_end),
                    reference_kind: ReferenceKind::Alias,
                });
            }
            MonValueKind::ArraySpread(name) => {
                table.add_reference(SymbolReference {
                    symbol_name: name.clone(),
                    symbol_kind: SymbolKind::Anchor,
                    range: Range::from_byte_offsets(source, value.pos_start, value.pos_end),
                    reference_kind: ReferenceKind::Spread,
                });
            }
            MonValueKind::Object(members) => {
                for member in members {
                    match member {
                        Member::TypeDefinition(typedef) => {
                            // Track type definitions
                            table.add_symbol(Symbol {
                                name: typedef.name.clone(),
                                kind: SymbolKind::Type,
                                range: Range::from_byte_offsets(
                                    source,
                                    value.pos_start,
                                    value.pos_end,
                                ),
                                detail: Some(format!("{:?}", typedef.def_type)),
                                documentation: None,
                            });
                        }
                        Member::Pair(pair) => {
                            // Recurse into pair values
                            self.build_symbols_from_value(&pair.value, source, table);
                        }
                        Member::Spread(name) => {
                            // Track spread reference
                            table.add_reference(SymbolReference {
                                symbol_name: name.clone(),
                                symbol_kind: SymbolKind::Anchor,
                                range: Range::from_byte_offsets(
                                    source,
                                    value.pos_start,
                                    value.pos_end,
                                ),
                                reference_kind: ReferenceKind::Spread,
                            });
                        }
                        _ => {}
                    }
                }
            }
            MonValueKind::Array(items) => {
                for item in items {
                    self.build_symbols_from_value(item, source, table);
                }
            }
            _ => {}
        }
    }

    /// Gets a preview string for a value (for hover/detail text).
    fn get_value_preview(&self, value: &MonValue) -> Option<String> {
        match &value.kind {
            MonValueKind::Object(_) => Some("{ ... }".to_string()),
            MonValueKind::Array(_) => Some("[ ... ]".to_string()),
            MonValueKind::String(s) => Some(format!("\"{}\"", s)),
            MonValueKind::Number(n) => Some(n.to_string()),
            MonValueKind::Boolean(b) => Some(b.to_string()),
            MonValueKind::Null => Some("null".to_string()),
            _ => None,
        }
    }

    /// Detects unused symbols and adds diagnostics.
    fn detect_unused_symbols(&self, table: &SymbolTable, source: &str, result: &mut LintResult) {
        if self.config.warn_unused_anchors {
            for symbol in table.find_unused_symbols(SymbolKind::Anchor) {
                result.add_diagnostic_with_range(
                    crate::linter::diagnostic::DiagnosticCode::UnusedAnchor,
                    format!("Anchor '{}' is defined but never used", symbol.name),
                    Some(symbol.range),
                    vec![],
                    vec![crate::linter::DiagnosticTag::Unnecessary],
                );
            }
        }
    }
}

/// Result of analyzing a MON document.
///
/// Contains diagnostics, symbol table, and the original source for reference.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// All diagnostics found
    pub diagnostics: Vec<crate::linter::Diagnostic>,

    /// Symbol table for LSP features
    pub symbol_table: SymbolTable,

    /// Original source text
    pub source: String,
}

impl AnalysisResult {
    /// Checks if there are any error-level diagnostics.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, crate::linter::DiagnosticSeverity::Error))
    }

    /// Gets all error diagnostics.
    pub fn errors(&self) -> Vec<&crate::linter::Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, crate::linter::DiagnosticSeverity::Error))
            .collect()
    }

    /// Gets all warning diagnostics.
    pub fn warnings(&self) -> Vec<&crate::linter::Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, crate::linter::DiagnosticSeverity::Warning))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_service_basic() {
        let source = r#"{
            &test: { value: 1 },
            data: *test
        }"#;

        let service = AnalysisService::new(LintConfig::default());
        let result = service.analyze_document(source, "test.mon").unwrap();

        // Should have found the anchor
        assert_eq!(result.symbol_table.symbol_count(), 1);

        // Should have one reference
        assert_eq!(result.symbol_table.reference_count(), 1);

        // Anchor should not be unused (it's referenced)
        assert!(!result.symbol_table.is_unused("test", SymbolKind::Anchor));
    }

    #[test]
    fn test_unused_anchor_detection() {
        let source = r#"{
            &unused: { value: 1 },
            data: { x: 2 }
        }"#;

        let config = LintConfig { warn_unused_anchors: true, ..Default::default() };

        let service = AnalysisService::new(config);
        let result = service.analyze_document(source, "test.mon").unwrap();

        // Should detect unused anchor
        assert!(result.symbol_table.is_unused("unused", SymbolKind::Anchor));

        // Should have diagnostic for unused anchor
        let has_unused_diagnostic = result
            .diagnostics
            .iter()
            .any(|d| matches!(d.code, crate::linter::diagnostic::DiagnosticCode::UnusedAnchor));
        assert!(has_unused_diagnostic);
    }
}
