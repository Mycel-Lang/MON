use serde::{Deserialize, Serialize};

/// Comprehensive formatting configuration
/// Designed to support multiple coding styles and explicit options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    // === INDENTATION ===
    /// Indentation style: "spaces" or "tabs"
    #[serde(default = "default_indent_style")]
    pub indent_style: IndentStyle,

    /// Number of spaces per indent (if indent_style is Spaces)
    #[serde(default = "default_indent_size")]
    pub indent_size: usize,

    // === LINE WIDTH ===
    /// Maximum line width before breaking
    #[serde(default = "default_max_line_width")]
    pub max_line_width: usize,

    // === OBJECT FORMATTING ===
    /// Object formatting style
    #[serde(default = "default_object_style")]
    pub object_style: ObjectStyle,

    /// Force objects to expand if they exceed this many properties
    #[serde(default = "default_object_expand_threshold")]
    pub object_expand_threshold: usize,

    /// Space before colon in objects
    #[serde(default = "default_space_before_colon")]
    pub space_before_colon: bool,

    /// Space after colon in objects
    #[serde(default = "default_space_after_colon")]
    pub space_after_colon: bool,

    // === ARRAY FORMATTING ===
    /// Array formatting style
    #[serde(default = "default_array_style")]
    pub array_style: ArrayStyle,

    /// Force arrays to expand if they exceed this many items
    #[serde(default = "default_array_expand_threshold")]
    pub array_expand_threshold: usize,

    /// Space inside array brackets
    #[serde(default = "default_space_in_brackets")]
    pub space_in_brackets: bool,

    // === COMMA HANDLING ===
    /// Trailing commas in objects and arrays
    #[serde(default = "default_trailing_commas")]
    pub trailing_commas: TrailingCommaStyle,

    // === STRING FORMATTING ===
    /// Quote style for strings
    #[serde(default = "default_quote_style")]
    pub quote_style: QuoteStyle,

    // === COMMENT HANDLING ===
    /// How to handle inline comments
    #[serde(default = "default_comment_placement")]
    pub comment_placement: CommentPlacement,

    // === SPECIAL CASES ===
    /// Keep empty objects on single line: {}
    #[serde(default = "default_single_line_empty_objects")]
    pub single_line_empty_objects: bool,

    /// Keep empty arrays on single line: []
    #[serde(default = "default_single_line_empty_arrays")]
    pub single_line_empty_arrays: bool,

    /// Newline at end of file
    #[serde(default = "default_final_newline")]
    pub final_newline: bool,

    // --- Advanced Formatting Options ---
    /// Align trailing comments at a consistent column
    #[serde(default = "default_align_trailing_comments")]
    pub align_trailing_comments: bool,

    /// Column for comment alignment (0 = auto-detect)
    #[serde(default = "default_comment_alignment_column")]
    pub comment_alignment_column: usize,

    /// Sort object keys
    #[serde(default = "default_sort_keys")]
    pub sort_keys: KeySortStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IndentStyle {
    Spaces,
    Tabs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ObjectStyle {
    /// Always expand objects to multiple lines
    Expanded,
    /// Keep small objects on single line
    Compact,
    /// Auto-decide based on content
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArrayStyle {
    /// Always expand arrays to multiple lines
    Expanded,
    /// Keep small arrays on single line
    Compact,
    /// Auto-decide based on content
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeySortStyle {
    None,
    Alpha,
    Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrailingCommaStyle {
    /// Always add trailing commas
    Always,
    /// Never add trailing commas
    Never,
    /// Add trailing commas only in multiline contexts
    Multiline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum QuoteStyle {
    /// Use double quotes (default)
    Double,
    /// Use single quotes (not standard MON, but configurable)
    Single,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommentPlacement {
    /// Preserve exactly where comments are
    Preserve,
    /// Move inline comments to end of line
    EndOfLine,
    /// Move all comments to own line above
    OwnLine,
}

// === DEFAULT VALUES ===

fn default_indent_style() -> IndentStyle {
    IndentStyle::Spaces
}

fn default_indent_size() -> usize {
    4
}

fn default_max_line_width() -> usize {
    80
}

fn default_object_style() -> ObjectStyle {
    ObjectStyle::Auto
}

fn default_object_expand_threshold() -> usize {
    3
}

fn default_space_before_colon() -> bool {
    false
}

fn default_space_after_colon() -> bool {
    true
}

fn default_array_style() -> ArrayStyle {
    ArrayStyle::Auto
}

fn default_array_expand_threshold() -> usize {
    5
}

fn default_space_in_brackets() -> bool {
    false
}

fn default_trailing_commas() -> TrailingCommaStyle {
    TrailingCommaStyle::Multiline
}

fn default_quote_style() -> QuoteStyle {
    QuoteStyle::Double
}

fn default_comment_placement() -> CommentPlacement {
    CommentPlacement::Preserve
}

fn default_single_line_empty_objects() -> bool {
    true
}

fn default_single_line_empty_arrays() -> bool {
    true
}

fn default_final_newline() -> bool {
    true
}

fn default_align_trailing_comments() -> bool {
    false
}

fn default_comment_alignment_column() -> usize {
    0 // 0 means auto-detect
}

fn default_sort_keys() -> KeySortStyle {
    KeySortStyle::None
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            indent_style: default_indent_style(),
            indent_size: default_indent_size(),
            max_line_width: default_max_line_width(),
            object_style: default_object_style(),
            object_expand_threshold: default_object_expand_threshold(),
            space_before_colon: default_space_before_colon(),
            space_after_colon: default_space_after_colon(),
            array_style: default_array_style(),
            array_expand_threshold: default_array_expand_threshold(),
            space_in_brackets: default_space_in_brackets(),
            trailing_commas: default_trailing_commas(),
            quote_style: default_quote_style(),
            comment_placement: default_comment_placement(),
            single_line_empty_objects: default_single_line_empty_objects(),
            single_line_empty_arrays: default_single_line_empty_arrays(),
            final_newline: default_final_newline(),
            align_trailing_comments: default_align_trailing_comments(),
            comment_alignment_column: default_comment_alignment_column(),
            sort_keys: default_sort_keys(),
        }
    }
}

impl FormatConfig {
    /// Load configuration from a MON file
    pub fn from_mon_file(path: &str) -> miette::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| miette::miette!("Failed to read config file: {}", e))?;

        // Parse MON file and analyze
        let result = mon_core::api::analyze(&content, path)
            .map_err(|e| miette::miette!("Failed to parse config file: {:?}", e))?;

        // Convert to JSON for serde deserialization
        let json_str =
            result.to_json().map_err(|e| miette::miette!("Failed to serialize config: {}", e))?;

        let config: Self = serde_json::from_str(&json_str)
            .map_err(|e| miette::miette!("Failed to deserialize config: {}", e))?;

        Ok(config)
    }

    /// Get indent string based on configuration
    pub fn indent_string(&self) -> String {
        match self.indent_style {
            IndentStyle::Spaces => " ".repeat(self.indent_size),
            IndentStyle::Tabs => "\t".to_string(),
        }
    }
}
