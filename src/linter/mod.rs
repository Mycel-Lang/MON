// Linter module - Code quality analysis for MON files

pub mod api;
pub mod complexity;
pub mod diagnostic;
pub mod imports;
pub mod position;
pub mod rules;
pub mod smells;
pub mod symbol_table;

#[cfg(test)]
mod tests;

pub use complexity::ComplexityAnalyzer;
pub use diagnostic::{DiagnosticCode, DiagnosticSeverity};
pub use imports::ImportAnalyzer;
pub use position::{DiagnosticTag, Range, RelatedInformation};
pub use smells::SmellDetector;
pub use symbol_table::{ReferenceKind, Symbol, SymbolKind, SymbolReference, SymbolTable};

use miette::Result;
use mon_core::ast::MonDocument;
use serde::Serialize;

/// Result of linting a MON document.
///
/// Contains all diagnostics (errors, warnings, hints) found during analysis.
/// Can be serialized to JSON for LSP or CLI output.
#[derive(Debug, Clone, Serialize)]
pub struct LintResult {
    /// All diagnostics found in the document
    pub diagnostics: Vec<Diagnostic>,
}

/// A diagnostic message (error, warning, or hint) from the linter.
///
/// This structure is LSP-compatible and includes precise location information,
/// related context, and tags for special rendering.
///
/// # Examples
/// ```ignore
/// let diagnostic = Diagnostic {
///     code: DiagnosticCode::UnusedAnchor,
///     code_name: "UnusedAnchor".to_string(),
///     severity: DiagnosticSeverity::Warning,
///     message: "Anchor 'foo' is defined but never used".to_string(),
///     range: Some(Range::new(Position::new(1, 5), Position::new(1, 8))),
///     related_information: vec![],
///     tags: vec![DiagnosticTag::Unnecessary],
/// };
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    /// The diagnostic code (e.g., "LINT2001")
    pub code: DiagnosticCode,

    /// Human-readable name of the diagnostic code (e.g., "UnusedAnchor")
    #[serde(rename = "code_name")]
    pub code_name: String,

    /// Severity level (Error, Warning, or Info)
    pub severity: DiagnosticSeverity,

    /// The diagnostic message
    pub message: String,

    /// The exact range in the source where the issue occurs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,

    /// Additional context (e.g., "defined here", "used here")
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub related_information: Vec<RelatedInformation>,

    /// Tags affecting rendering (e.g., strike-through for unused code)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<DiagnosticTag>,

    /// Legacy location string (deprecated, use range instead)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl LintResult {
    pub fn new() -> Self {
        Self { diagnostics: Vec::new() }
    }

    pub fn has_issues(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Adds a diagnostic with optional range and related information.
    ///
    /// # Arguments
    /// * `code` - The diagnostic code
    /// * `message` - The diagnostic message
    /// * `range` - Optional precise location in the source
    /// * `related_info` - Optional additional context
    /// * `tags` - Optional tags (unnecessary, deprecated)
    pub fn add_diagnostic_with_range(
        &mut self,
        code: DiagnosticCode,
        message: String,
        range: Option<Range>,
        related_info: Vec<RelatedInformation>,
        tags: Vec<DiagnosticTag>,
    ) {
        self.diagnostics.push(Diagnostic {
            severity: code.severity(),
            code_name: format!("{:?}", code),
            code,
            message,
            range,
            related_information: related_info,
            tags,
            location: None,
        });
    }

    /// Legacy method: adds a diagnostic with optional location string.
    ///
    /// Prefer `add_diagnostic_with_range` for new code.
    pub fn add_diagnostic(
        &mut self,
        code: DiagnosticCode,
        message: String,
        location: Option<String>,
    ) {
        self.diagnostics.push(Diagnostic {
            severity: code.severity(),
            code_name: format!("{:?}", code),
            code,
            message,
            range: None,
            related_information: vec![],
            tags: vec![],
            location,
        });
    }

    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .collect()
    }

    pub fn warnings(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Warning))
            .collect()
    }

    pub fn infos(&self) -> Vec<&Diagnostic> {
        self.diagnostics.iter().filter(|d| matches!(d.severity, DiagnosticSeverity::Info)).collect()
    }

    // Legacy compatibility methods
    pub fn add_warning(&mut self, _rule: &str, message: String, location: Option<String>) {
        // Map old warnings to new system - need to infer code from message
        let code = if message.contains("nesting") {
            DiagnosticCode::MaxNestingDepth
        } else if message.contains("members") {
            DiagnosticCode::MaxObjectMembers
        } else if message.contains("items") {
            DiagnosticCode::MaxArrayItems
        } else {
            DiagnosticCode::UnusedAnchor
        };
        self.add_diagnostic(code, message, location);
    }

    pub fn add_error(&mut self, _rule: &str, message: String, location: Option<String>) {
        let code = if message.contains("Duplicate") {
            DiagnosticCode::DuplicateKey
        } else {
            DiagnosticCode::CircularDependency
        };
        self.add_diagnostic(code, message, location);
    }

    pub fn add_info(&mut self, _rule: &str, message: String) {
        self.add_diagnostic(DiagnosticCode::MagicNumber, message, None);
    }
}

pub struct Linter {
    config: LintConfig,
}

#[derive(Debug, Clone)]
pub struct LintConfig {
    pub max_nesting_depth: usize,
    pub max_object_members: usize,
    pub max_array_items: usize,
    pub max_import_chain_depth: usize,
    pub warn_unused_anchors: bool,
    pub warn_magic_numbers: bool,
    pub suggest_type_validation: bool,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            max_nesting_depth: 4,
            max_object_members: 20,
            max_array_items: 100,
            max_import_chain_depth: 2,
            warn_unused_anchors: true,
            warn_magic_numbers: false,
            suggest_type_validation: false,
        }
    }
}

impl LintConfig {
    /// Create a config that only runs specified rules
    pub fn with_only_rules(self, _rules: Vec<String>) -> Self {
        // TODO: Implement rule filtering
        // For MVP, just return self
        self
    }

    /// Create a config that disables specified rules
    pub fn without_rules(self, _rules: Vec<String>) -> Self {
        // TODO: Implement rule filtering
        // For MVP, just return self
        self
    }

    /// Load config from a MonDocument
    pub fn from_document(_doc: &MonDocument) -> Result<Self> {
        // TODO: Parse config from MonDocument
        // For MVP, just return default
        Ok(Self::default())
    }
}

impl Linter {
    pub fn new(config: LintConfig) -> Self {
        Self { config }
    }

    pub fn lint(&self, doc: &MonDocument, _source: &str) -> Result<LintResult> {
        let mut result = LintResult::new();

        // Run complexity analysis
        let complexity_analyzer = ComplexityAnalyzer::new(self.config.clone());
        complexity_analyzer.analyze(&doc.root, &mut result);

        // Run smell detection
        let smell_detector =
            SmellDetector::new(self.config.warn_unused_anchors, self.config.warn_magic_numbers);
        smell_detector.detect(&doc.root, &mut result);

        // Run import analysis
        let import_analyzer = ImportAnalyzer::new(self.config.clone());
        import_analyzer.analyze(&doc.root, &mut result);

        Ok(result)
    }
}
