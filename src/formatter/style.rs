/// Predefined coding styles for quick selection
use super::config::*;

impl FormatConfig {
    /// Google-style configuration
    /// - 2 spaces
    /// - Compact style
    /// - Trailing commas in multiline
    pub fn google_style() -> Self {
        Self {
            indent_size: 2,
            object_style: ObjectStyle::Compact,
            array_style: ArrayStyle::Compact,
            trailing_commas: TrailingCommaStyle::Multiline,
            ..Default::default()
        }
    }

    /// Mozilla-style configuration
    /// - 2 spaces
    /// - Expanded style
    /// - Always trailing commas
    pub fn mozilla_style() -> Self {
        Self {
            indent_size: 2,
            object_style: ObjectStyle::Expanded,
            array_style: ArrayStyle::Expanded,
            trailing_commas: TrailingCommaStyle::Always,
            ..Default::default()
        }
    }

    /// Airbnb-style configuration
    /// - 2 spaces
    /// - Auto style
    /// - Trailing commas in multiline
    /// - Space after colon
    pub fn airbnb_style() -> Self {
        Self {
            indent_size: 2,
            object_style: ObjectStyle::Auto,
            array_style: ArrayStyle::Auto,
            trailing_commas: TrailingCommaStyle::Multiline,
            space_after_colon: true,
            ..Default::default()
        }
    }

    /// Linux kernel style (adapted)
    /// - Tabs
    /// - Compact
    /// - No trailing commas
    pub fn linux_style() -> Self {
        Self {
            indent_style: IndentStyle::Tabs,
            object_style: ObjectStyle::Compact,
            array_style: ArrayStyle::Compact,
            trailing_commas: TrailingCommaStyle::Never,
            max_line_width: 100,
            ..Default::default()
        }
    }

    /// Rust-style configuration (rustfmt-like)
    /// - 4 spaces
    /// - Auto style
    /// - Trailing commas in multiline
    /// - 100 char line width
    pub fn rust_style() -> Self {
        Self {
            indent_size: 4,
            object_style: ObjectStyle::Auto,
            array_style: ArrayStyle::Auto,
            trailing_commas: TrailingCommaStyle::Multiline,
            max_line_width: 100,
            ..Default::default()
        }
    }

    /// Prettier-style configuration
    /// - 2 spaces
    /// - Auto style
    /// - Trailing commas in multiline  
    /// - 80 char line width
    pub fn prettier_style() -> Self {
        Self {
            indent_size: 2,
            object_style: ObjectStyle::Auto,
            array_style: ArrayStyle::Auto,
            trailing_commas: TrailingCommaStyle::Multiline,
            max_line_width: 80,
            ..Default::default()
        }
    }
}

/// Style presets
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Google,
    Mozilla,
    Airbnb,
    Linux,
    Rust,
    Prettier,
    Default,
}

impl Style {
    pub fn to_config(self) -> FormatConfig {
        match self {
            Style::Google => FormatConfig::google_style(),
            Style::Mozilla => FormatConfig::mozilla_style(),
            Style::Airbnb => FormatConfig::airbnb_style(),
            Style::Linux => FormatConfig::linux_style(),
            Style::Rust => FormatConfig::rust_style(),
            Style::Prettier => FormatConfig::prettier_style(),
            Style::Default => FormatConfig::default(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "google" => Some(Style::Google),
            "mozilla" => Some(Style::Mozilla),
            "airbnb" => Some(Style::Airbnb),
            "linux" => Some(Style::Linux),
            "rust" => Some(Style::Rust),
            "prettier" => Some(Style::Prettier),
            "default" => Some(Style::Default),
            _ => None,
        }
    }
}
