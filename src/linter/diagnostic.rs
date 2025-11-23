// Lint diagnostic codes and metadata

use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    // Complexity warnings (LINT1xxx)
    MaxNestingDepth,  // LINT1001
    MaxObjectMembers, // LINT1002
    MaxArrayItems,    // LINT1003

    // Code smell warnings (LINT2xxx)
    UnusedAnchor,     // LINT2001
    DuplicateKey,     // LINT2002
    ExcessiveSpreads, // LINT2003
    MagicNumber,      // LINT2004

    // Best practices (LINT3xxx) - Future implementation
    #[allow(dead_code)]
    MissingTypeValidation, // LINT3001
    #[allow(dead_code)]
    InconsistentNaming, // LINT3002
    #[allow(dead_code)]
    EmptyObject, // LINT3003

    // Import issues (LINT4xxx) - Future implementation
    #[allow(dead_code)]
    DeepImportChain, // LINT4001
    #[allow(dead_code)]
    CircularDependency, // LINT4002
    #[allow(dead_code)]
    UnusedImport, // LINT4003
}

// Custom serialization to output error codes instead of variant names
impl Serialize for DiagnosticCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.code())
    }
}

impl DiagnosticCode {
    pub fn code(&self) -> &'static str {
        match self {
            DiagnosticCode::MaxNestingDepth => "LINT1001",
            DiagnosticCode::MaxObjectMembers => "LINT1002",
            DiagnosticCode::MaxArrayItems => "LINT1003",
            DiagnosticCode::UnusedAnchor => "LINT2001",
            DiagnosticCode::DuplicateKey => "LINT2002",
            DiagnosticCode::ExcessiveSpreads => "LINT2003",
            DiagnosticCode::MagicNumber => "LINT2004",
            DiagnosticCode::MissingTypeValidation => "LINT3001",
            DiagnosticCode::InconsistentNaming => "LINT3002",
            DiagnosticCode::EmptyObject => "LINT3003",
            DiagnosticCode::DeepImportChain => "LINT4001",
            DiagnosticCode::CircularDependency => "LINT4002",
            DiagnosticCode::UnusedImport => "LINT4003",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            DiagnosticCode::MaxNestingDepth => "Excessive nesting depth",
            DiagnosticCode::MaxObjectMembers => "Too many object members",
            DiagnosticCode::MaxArrayItems => "Too many array items",
            DiagnosticCode::UnusedAnchor => "Unused anchor definition",
            DiagnosticCode::DuplicateKey => "Duplicate object key",
            DiagnosticCode::ExcessiveSpreads => "Too many spread operators",
            DiagnosticCode::MagicNumber => "Magic number literal",
            DiagnosticCode::MissingTypeValidation => "Missing type validation",
            DiagnosticCode::InconsistentNaming => "Inconsistent naming convention",
            DiagnosticCode::EmptyObject => "Empty object or array",
            DiagnosticCode::DeepImportChain => "Deep import chain",
            DiagnosticCode::CircularDependency => "Circular dependency detected",
            DiagnosticCode::UnusedImport => "Unused import",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DiagnosticCode::MaxNestingDepth => {
                "Deeply nested structures are hard to read and maintain. Consider flattening or extracting nested parts."
            }
            DiagnosticCode::MaxObjectMembers => {
                "Large objects with many members are difficult to understand. Consider splitting into smaller, focused objects."
            }
            DiagnosticCode::MaxArrayItems => {
                "Very large arrays may indicate the need for pagination or chunking. Consider restructuring your data."
            }
            DiagnosticCode::UnusedAnchor => {
                "An anchor is defined but never referenced. Remove it or use it with an alias (*anchor) or spread (...*anchor)."
            }
            DiagnosticCode::DuplicateKey => {
                "Object has duplicate keys. The second occurrence will override the first, which is likely unintentional."
            }
            DiagnosticCode::ExcessiveSpreads => {
                "Too many spread operators in a single object make it hard to track the final shape. Consider simplifying."
            }
            DiagnosticCode::MagicNumber => {
                "Literal numbers without context are hard to understand. Extract them as named constants with descriptive names."
            }
            DiagnosticCode::MissingTypeValidation => {
                "Data lacks type validation. Add type constraints (:: TypeName) to ensure data integrity."
            }
            DiagnosticCode::InconsistentNaming => {
                "Keys use inconsistent naming conventions (camelCase vs snake_case). Choose one style for consistency."
            }
            DiagnosticCode::EmptyObject => {
                "Empty objects or arrays may indicate incomplete data or unnecessary structure. Verify this is  intentional."
            }
            DiagnosticCode::DeepImportChain => {
                "Anchor or type is imported through multiple levels. This creates tight coupling and makes refactoring difficult."
            }
            DiagnosticCode::CircularDependency => {
                "Files import each other in a cycle. This can cause issues and indicates poor module organization."
            }
            DiagnosticCode::UnusedImport => {
                "Import statement brings in items that are never used. Remove to keep code clean."
            }
        }
    }

    pub fn severity(&self) -> DiagnosticSeverity {
        match self {
            DiagnosticCode::DuplicateKey => DiagnosticSeverity::Error,
            DiagnosticCode::CircularDependency => DiagnosticSeverity::Error,

            DiagnosticCode::MaxNestingDepth => DiagnosticSeverity::Warning,
            DiagnosticCode::MaxObjectMembers => DiagnosticSeverity::Warning,
            DiagnosticCode::MaxArrayItems => DiagnosticSeverity::Warning,
            DiagnosticCode::UnusedAnchor => DiagnosticSeverity::Warning,
            DiagnosticCode::ExcessiveSpreads => DiagnosticSeverity::Warning,
            DiagnosticCode::DeepImportChain => DiagnosticSeverity::Warning,
            DiagnosticCode::UnusedImport => DiagnosticSeverity::Warning,

            DiagnosticCode::MagicNumber => DiagnosticSeverity::Info,
            DiagnosticCode::MissingTypeValidation => DiagnosticSeverity::Info,
            DiagnosticCode::InconsistentNaming => DiagnosticSeverity::Info,
            DiagnosticCode::EmptyObject => DiagnosticSeverity::Info,
        }
    }

    pub fn config_key(&self) -> &'static str {
        match self {
            DiagnosticCode::MaxNestingDepth => "max_nesting_depth",
            DiagnosticCode::MaxObjectMembers => "max_object_members",
            DiagnosticCode::MaxArrayItems => "max_array_items",
            DiagnosticCode::UnusedAnchor => "warn_unused_anchors",
            DiagnosticCode::DuplicateKey => "N/A (always enabled)",
            DiagnosticCode::ExcessiveSpreads => "max_spreads_per_object",
            DiagnosticCode::MagicNumber => "warn_magic_numbers",
            DiagnosticCode::MissingTypeValidation => "suggest_type_validation",
            DiagnosticCode::InconsistentNaming => "enforce_naming_convention",
            DiagnosticCode::EmptyObject => "warn_empty_structures",
            DiagnosticCode::DeepImportChain => "max_import_chain_depth",
            DiagnosticCode::CircularDependency => "N/A (always enabled)",
            DiagnosticCode::UnusedImport => "warn_unused_imports",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

impl DiagnosticSeverity {
    pub fn short_label(&self) -> &'static str {
        match self {
            DiagnosticSeverity::Error => "[E]",
            DiagnosticSeverity::Warning => "[W]",
            DiagnosticSeverity::Info => "[I]",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            DiagnosticSeverity::Error => "error",
            DiagnosticSeverity::Warning => "warning",
            DiagnosticSeverity::Info => "info",
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}
