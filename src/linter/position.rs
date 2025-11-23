//! Position and range types for LSP-compatible location tracking.
//!
//! This module provides types for representing positions and ranges in source code
//! using zero-based line and column indices, compatible with LSP and most editors.

use serde::{Deserialize, Serialize};

/// A position in a text document, using zero-based line and column indices.
///
/// This follows the LSP specification where:
/// - Lines are zero-indexed (first line is 0)
/// - Characters are zero-indexed (first char is 0)
/// - Character offsets are UTF-16 code units (for LSP compatibility)
///
/// # Examples
/// ```
/// # use mon::linter::Position;
/// let pos = Position { line: 0, character: 5 };
/// assert_eq!(pos.line, 0);
/// assert_eq!(pos.character, 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Zero-based line number
    pub line: u32,

    /// Zero-based character offset (UTF-16 code units for LSP)
    pub character: u32,
}

impl Position {
    /// Creates a new position.
    ///
    /// # Arguments
    /// * `line` - Zero-based line number
    /// * `character` - Zero-based character offset
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }

    /// Converts byte offset in source to line/column position.
    ///
    /// This is used to convert mon-core's byte offsets to editor positions.
    ///
    /// # Arguments
    /// * `source` - The full source text
    /// * `byte_offset` - Byte offset in the source
    ///
    /// # Returns
    /// Position at the given byte offset, or (0, 0) if offset is invalid
    pub fn from_byte_offset(source: &str, byte_offset: usize) -> Self {
        let mut line = 0;
        let mut character = 0;

        for (idx, ch) in source.char_indices() {
            if idx >= byte_offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += ch.len_utf16() as u32;
            }
        }

        Position { line, character }
    }
}

/// A range in a text document, defined by start and end positions.
///
/// The range is inclusive of the start position and exclusive of the end position,
/// following LSP conventions.
///
/// # Examples
/// ```
/// # use mon::linter::{Position, Range};
/// let range = Range {
///     start: Position { line: 0, character: 0 },
///     end: Position { line: 0, character: 5 },
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// The range's start position (inclusive)
    pub start: Position,

    /// The range's end position (exclusive)
    pub end: Position,
}

impl Range {
    /// Creates a new range.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Creates a range from byte offsets in source text.
    ///
    /// # Arguments
    /// * `source` - The full source text
    /// * `start_offset` - Starting byte offset
    /// * `end_offset` - Ending byte offset
    pub fn from_byte_offsets(source: &str, start_offset: usize, end_offset: usize) -> Self {
        Self {
            start: Position::from_byte_offset(source, start_offset),
            end: Position::from_byte_offset(source, end_offset),
        }
    }

    /// Checks if this range contains a given position.
    pub fn contains(&self, position: Position) -> bool {
        if position.line < self.start.line || position.line > self.end.line {
            return false;
        }

        if position.line == self.start.line && position.character < self.start.character {
            return false;
        }

        if position.line == self.end.line && position.character >= self.end.character {
            return false;
        }

        true
    }
}

/// Related information for a diagnostic, providing additional context.
///
/// This can be used to show where an anchor is defined when reporting it as unused,
/// or to show all locations where a duplicate key appears.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedInformation {
    /// The location of the related information
    pub location: Location,

    /// A human-readable message describing the relationship
    pub message: String,
}

/// A location in a source file, combining file path and range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// File URI (typically file:// path)
    pub uri: String,

    /// The range within the file
    pub range: Range,
}

impl Location {
    /// Creates a new location.
    pub fn new(uri: String, range: Range) -> Self {
        Self { uri, range }
    }
}

/// Tags for diagnostics that affect rendering or behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticTag {
    /// Unused or unnecessary code (e.g., unused import, unused anchor).
    /// Editors typically render this with strike-through or dimming.
    Unnecessary,

    /// Deprecated code that should be avoided.
    /// Editors typically render this with strike-through.
    Deprecated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_from_byte_offset() {
        let source = "hello\nworld\nfoo";

        // First line
        assert_eq!(Position::from_byte_offset(source, 0), Position { line: 0, character: 0 });
        assert_eq!(Position::from_byte_offset(source, 2), Position { line: 0, character: 2 });

        // Second line
        assert_eq!(Position::from_byte_offset(source, 6), Position { line: 1, character: 0 });
        assert_eq!(Position::from_byte_offset(source, 9), Position { line: 1, character: 3 });

        // Third line
        assert_eq!(Position::from_byte_offset(source, 12), Position { line: 2, character: 0 });
    }

    #[test]
    fn test_range_contains() {
        let range = Range {
            start: Position { line: 1, character: 5 },
            end: Position { line: 1, character: 10 },
        };

        assert!(range.contains(Position { line: 1, character: 5 }));
        assert!(range.contains(Position { line: 1, character: 7 }));
        assert!(!range.contains(Position { line: 1, character: 10 })); // Exclusive end
        assert!(!range.contains(Position { line: 0, character: 5 }));
        assert!(!range.contains(Position { line: 2, character: 5 }));
    }
}
