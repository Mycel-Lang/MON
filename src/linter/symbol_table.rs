//! Symbol table for tracking definitions and their locations.
//!
//! This module provides a symbol table that tracks all anchors, type definitions,
//! and imports in a MON document along with their source locations. This enables
//! LSP features like go-to-definition, find references, and hover information.

use crate::linter::position::Range;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A symbol in the MON document (anchor, type, or import).
///
/// # Examples
/// ```ignore
/// let symbol = Symbol {
///     name: "base_user".to_string(),
///     kind: SymbolKind::Anchor,
///     range: Range::new(Position::new(2, 4), Position::new(2, 13)),
///     detail: Some("{ id: 1, name: \"Alice\" }".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// The symbol name (e.g., "base_user", "User", "./types.mon")
    pub name: String,

    /// The kind of symbol
    pub kind: SymbolKind,

    /// The range where this symbol is defined
    pub range: Range,

    /// Optional detail text (e.g., type signature, value preview)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /// Optional documentation string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

/// The kind of symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    /// An anchor definition (&name)
    Anchor,

    /// A type definition (struct or enum)
    Type,

    /// An import statement
    Import,

    /// A struct field
    Field,

    /// An enum variant
    EnumVariant,
}

/// Reference to a symbol usage.
///
/// Tracks where a symbol is referenced (used), as opposed to where it's defined.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolReference {
    /// The symbol name being referenced
    pub symbol_name: String,

    /// The kind of symbol
    pub symbol_kind: SymbolKind,

    /// The range where the symbol is used
    pub range: Range,

    /// The kind of reference (alias, spread, type annotation, etc.)
    pub reference_kind: ReferenceKind,
}

/// The kind of reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceKind {
    /// Alias usage (*anchor)
    Alias,

    /// Spread usage (...*anchor)
    Spread,

    /// Type annotation (:: Type)
    TypeAnnotation,

    /// Import usage
    Import,
}

/// Symbol table tracking all definitions and references in a document.
///
/// This table is used by the LSP for features like:
/// - Go to definition
/// - Find all references
/// - Hover information
/// - Document symbols (outline view)
///
/// # Example
/// ```ignore
/// let mut table = SymbolTable::new();
///
/// // Add an anchor definition
/// table.add_symbol(Symbol {
///     name: "base_user".to_string(),
///     kind: SymbolKind::Anchor,
///     range: Range::new(Position::new(2, 4), Position::new(2, 13)),
///     detail: Some("{ id: 1 }".to_string()),
/// });
///
/// // Add a reference to that anchor
/// table.add_reference(SymbolReference {
///     symbol_name: "base_user".to_string(),
///     symbol_kind: SymbolKind::Anchor,
///     range: Range::new(Position::new(5, 10), Position::new(5, 19)),
///     reference_kind: ReferenceKind::Alias,
/// });
///
/// // Find all references to the anchor
/// let refs = table.find_references("base_user", SymbolKind::Anchor);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolTable {
    /// All symbol definitions, keyed by (name, kind)
    symbols: HashMap<(String, SymbolKind), Symbol>,

    /// All symbol references, keyed by (symbol_name, symbol_kind)
    references: HashMap<(String, SymbolKind), Vec<SymbolReference>>,
}

impl SymbolTable {
    /// Creates a new empty symbol table.
    pub fn new() -> Self {
        Self { symbols: HashMap::new(), references: HashMap::new() }
    }

    /// Adds a symbol definition to the table.
    ///
    /// If a symbol with the same name and kind already exists, it will be replaced.
    pub fn add_symbol(&mut self, symbol: Symbol) {
        let key = (symbol.name.clone(), symbol.kind);
        self.symbols.insert(key, symbol);
    }

    /// Adds a symbol reference to the table.
    pub fn add_reference(&mut self, reference: SymbolReference) {
        let key = (reference.symbol_name.clone(), reference.symbol_kind);
        self.references.entry(key).or_default().push(reference);
    }

    /// Finds a symbol definition by name and kind.
    ///
    /// # Returns
    /// The symbol if found, None otherwise.
    pub fn find_symbol(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        self.symbols.get(&(name.to_string(), kind))
    }

    /// Finds all references to a symbol.
    ///
    /// # Returns
    /// A slice of all references, or an empty slice if none found.
    pub fn find_references(&self, name: &str, kind: SymbolKind) -> &[SymbolReference] {
        self.references.get(&(name.to_string(), kind)).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Gets all symbols of a specific kind.
    ///
    /// Useful for "document symbols" LSP feature (outline view).
    pub fn symbols_by_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        self.symbols.iter().filter(|((_, k), _)| *k == kind).map(|(_, symbol)| symbol).collect()
    }

    /// Checks if a symbol is unused (defined but never referenced).
    pub fn is_unused(&self, name: &str, kind: SymbolKind) -> bool {
        self.find_symbol(name, kind).is_some() && self.find_references(name, kind).is_empty()
    }

    /// Gets all unused symbols.
    ///
    /// Useful for "unused anchor" and "unused import" diagnostics.
    pub fn find_unused_symbols(&self, kind: SymbolKind) -> Vec<&Symbol> {
        self.symbols
            .iter()
            .filter(|((name, k), _)| *k == kind && self.is_unused(name, kind))
            .map(|(_, symbol)| symbol)
            .collect()
    }

    /// Clears all symbols and references.
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.references.clear();
    }

    /// Gets the total number of symbols.
    pub fn symbol_count(&self) -> usize {
        self.symbols.len()
    }

    /// Gets the total number of references.
    pub fn reference_count(&self) -> usize {
        self.references.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::position::Position;

    #[test]
    fn test_symbol_table_basic() {
        let mut table = SymbolTable::new();

        let symbol = Symbol {
            name: "test_anchor".to_string(),
            kind: SymbolKind::Anchor,
            range: Range::new(Position::new(0, 0), Position::new(0, 5)),
            detail: None,
            documentation: None,
        };

        table.add_symbol(symbol.clone());

        assert_eq!(table.symbol_count(), 1);
        assert!(table.find_symbol("test_anchor", SymbolKind::Anchor).is_some());
        assert!(table.find_symbol("test_anchor", SymbolKind::Type).is_none());
    }

    #[test]
    fn test_find_references() {
        let mut table = SymbolTable::new();

        table.add_symbol(Symbol {
            name: "anchor".to_string(),
            kind: SymbolKind::Anchor,
            range: Range::new(Position::new(0, 0), Position::new(0, 5)),
            detail: None,
            documentation: None,
        });

        table.add_reference(SymbolReference {
            symbol_name: "anchor".to_string(),
            symbol_kind: SymbolKind::Anchor,
            range: Range::new(Position::new(1, 0), Position::new(1, 5)),
            reference_kind: ReferenceKind::Alias,
        });

        table.add_reference(SymbolReference {
            symbol_name: "anchor".to_string(),
            symbol_kind: SymbolKind::Anchor,
            range: Range::new(Position::new(2, 0), Position::new(2, 5)),
            reference_kind: ReferenceKind::Spread,
        });

        let refs = table.find_references("anchor", SymbolKind::Anchor);
        assert_eq!(refs.len(), 2);
        assert_eq!(table.reference_count(), 2);
    }

    #[test]
    fn test_unused_symbols() {
        let mut table = SymbolTable::new();

        // Add used symbol
        table.add_symbol(Symbol {
            name: "used".to_string(),
            kind: SymbolKind::Anchor,
            range: Range::new(Position::new(0, 0), Position::new(0, 4)),
            detail: None,
            documentation: None,
        });
        table.add_reference(SymbolReference {
            symbol_name: "used".to_string(),
            symbol_kind: SymbolKind::Anchor,
            range: Range::new(Position::new(1, 0), Position::new(1, 4)),
            reference_kind: ReferenceKind::Alias,
        });

        // Add unused symbol
        table.add_symbol(Symbol {
            name: "unused".to_string(),
            kind: SymbolKind::Anchor,
            range: Range::new(Position::new(2, 0), Position::new(2, 6)),
            detail: None,
            documentation: None,
        });

        assert!(!table.is_unused("used", SymbolKind::Anchor));
        assert!(table.is_unused("unused", SymbolKind::Anchor));

        let unused = table.find_unused_symbols(SymbolKind::Anchor);
        assert_eq!(unused.len(), 1);
        assert_eq!(unused[0].name, "unused");
    }
}
