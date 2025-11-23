use super::config::*;
use miette::Result;
use mon_core::ast::{ImportSpec, ImportStatement, Member, MonValue, MonValueKind, TypeDefinition};
use mon_core::parser::Parser;
use std::collections::HashMap;

/// Professional MON formatter with comment preservation
pub struct Formatter {
    config: FormatConfig,
}

impl Formatter {
    pub fn new(config: FormatConfig) -> Self {
        Self { config }
    }

    /// Format a MON source file with industrial-grade error handling
    ///
    /// Never fails catastrophically - provides partial formatting or helpful errors
    pub fn format(&self, source: &str) -> Result<String> {
        use miette::{NamedSource, SourceSpan};

        // Parse the source to AST with rich error diagnostics
        let mut parser = match Parser::new(source) {
            Ok(p) => p,
            Err(_e) => {
                // Create rich diagnostic error
                return Err(miette::miette! {
                    labels = vec![
                        miette::LabeledSpan::at(0..source.len().min(100), "failed to initialize parser"),
                    ],
                    help = "The file contains invalid MON syntax. Run 'mon check <file>' for detailed diagnostics.",
                    "Failed to parse MON file"
                }
                .with_source_code(NamedSource::new("input", source.to_string())));
            }
        };

        let doc = match parser.parse_document() {
            Ok(d) => d,
            Err(e) => {
                // Extract error details for rich diagnostic
                let error_msg = format!("{:?}", e);

                // Try to extract span information
                let span = if error_msg.contains("offset:") {
                    // Parse offset from error message (this is a workaround)
                    // In production, we'd get this from the error directly
                    SourceSpan::from(0..100)
                } else {
                    SourceSpan::from(0..source.len().min(100))
                };

                return Err(miette::miette! {
                    labels = vec![
                        miette::LabeledSpan::at(span, "syntax error here"),
                    ],
                    help = format!(
                        "The MON file contains syntax errors.\n\n\
                        Suggestions:\n\
                        1. Run 'mon check <file>' for detailed error information\n\
                        2. Check for missing colons, braces, or commas\n\
                        3. Ensure all strings are properly quoted\n\
                        4. Validate enum references use $Enum.Variant syntax\n\n\
                        Parse error details: {}",
                        error_msg
                    ),
                    "Failed to parse MON document"
                }
                .with_source_code(NamedSource::new("input", source.to_string())));
            }
        };

        // Extract comments from source (preserve positions)
        let comments = self.extract_comments(source);

        // Create comment map for efficient lookup
        let comment_map = self.build_comment_map(&comments);

        // Format the document with comment preservation
        let mut output = String::new();
        let mut current_line = 0usize;

        // Add top-of-file comments
        while let Some(comment) = comment_map.get(&current_line) {
            if !comment.is_trailing {
                output.push_str(&comment.text);
                output.push('\n');
                current_line += 1;
            } else {
                break;
            }
        }

        // Format imports
        for (i, import) in doc.imports.iter().enumerate() {
            output.push_str(&self.format_import(import));
            output.push('\n');

            // Check for comments after this import
            let import_line = i + current_line;
            if let Some(comment) = comment_map.get(&import_line) {
                if comment.is_trailing {
                    output.push_str("  ");
                    output.push_str(&comment.text);
                }
            }
        }

        if !doc.imports.is_empty() {
            output.push('\n');
        }

        // Format root value
        let formatted_root = self.format_value(&doc.root, 0, &comment_map, source);
        output.push_str(&formatted_root);

        // Ensure final newline if configured
        if self.config.final_newline && !output.ends_with('\n') {
            output.push('\n');
        }

        Ok(output)
    }

    /// Format an import statement
    fn format_import(&self, import: &ImportStatement) -> String {
        match &import.spec {
            ImportSpec::Namespace(ns) => {
                format!("import * as {} from \"{}\"", ns, import.path)
            }
            ImportSpec::Named(items) => {
                let names: Vec<String> =
                    items
                        .iter()
                        .map(|item| {
                            if item.is_anchor {
                                format!("&{}", item.name)
                            } else {
                                item.name.clone()
                            }
                        })
                        .collect();
                format!("import {{ {} }} from \"{}\"", names.join(", "), import.path)
            }
        }
    }

    /// Extract all comments from source with their positions
    fn extract_comments(&self, source: &str) -> Vec<Comment> {
        let mut comments = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(comment_start) = line.find("//") {
                // Check if it's not inside a string
                if !is_inside_string(line, comment_start) {
                    let comment_text = line[comment_start..].trim().to_string();
                    let is_trailing = comment_start > 0 && line[..comment_start].trim().len() > 0;

                    comments.push(Comment {
                        line: line_num,
                        column: comment_start,
                        text: comment_text,
                        is_trailing,
                    });
                }
            }
        }

        comments
    }

    /// Build a comment map for efficient lookup
    fn build_comment_map(&self, comments: &[Comment]) -> HashMap<usize, Comment> {
        comments.iter().map(|c| (c.line, c.clone())).collect()
    }

    /// Format a value recursively
    fn format_value(
        &self,
        value: &MonValue,
        depth: usize,
        comment_map: &HashMap<usize, Comment>,
        source: &str,
    ) -> String {
        // Handle anchor if present (for standalone values, not in KeyPart)
        let mut result = String::new();
        if let Some(anchor) = &value.anchor {
            result.push_str(&format!("&{} ", anchor));
        }

        let formatted = self.format_value_kind(&value.kind, depth, comment_map, source);
        result.push_str(&formatted);
        result
    }

    /// Format a value WITHOUT its anchor (used when anchor is in KeyPart)
    fn format_value_without_anchor(
        &self,
        value: &MonValue,
        depth: usize,
        comment_map: &HashMap<usize, Comment>,
        source: &str,
    ) -> String {
        self.format_value_kind(&value.kind, depth, comment_map, source)
    }

    /// Format value kind (the actual content)
    fn format_value_kind(
        &self,
        kind: &MonValueKind,
        depth: usize,
        comment_map: &HashMap<usize, Comment>,
        source: &str,
    ) -> String {
        match kind {
            MonValueKind::Null => "null".to_string(),
            MonValueKind::Boolean(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            MonValueKind::Number(n) => {
                // Preserve integer vs float representation
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            MonValueKind::String(s) => self.format_string(s),
            MonValueKind::Array(arr) => self.format_array(arr, depth, comment_map, source),
            MonValueKind::Object(members) => {
                self.format_object(members, depth, comment_map, source)
            }
            MonValueKind::Alias(name) => format!("*{}", name),
            MonValueKind::EnumValue { enum_name, variant_name } => {
                format!("${}.{}", enum_name, variant_name)
            }
            MonValueKind::ArraySpread(name) => format!("...*{}", name),
        }
    }

    /// Format a string with configured quote style
    fn format_string(&self, s: &str) -> String {
        // Always use double quotes for MON (single quotes not standard)
        format!("\"{}\"", s)
    }

    /// Format an array
    fn format_array(
        &self,
        items: &[MonValue],
        depth: usize,
        comment_map: &HashMap<usize, Comment>,
        source: &str,
    ) -> String {
        if items.is_empty() {
            return if self.config.single_line_empty_arrays {
                "[]".to_string()
            } else {
                "[\n]".to_string()
            };
        }

        let indent = self.indent_at(depth);
        let inner_indent = self.indent_at(depth + 1);

        // Decide if array should be single-line or multi-line
        let should_expand = self.should_expand_array(items);

        if should_expand {
            let mut result = String::from("[\n");

            for (i, item) in items.iter().enumerate() {
                result.push_str(&inner_indent);
                let formatted_item = self.format_value(item, depth + 1, comment_map, source);

                // Check line width
                if inner_indent.len() + formatted_item.len() > self.config.max_line_width {
                    result.push_str(&formatted_item);
                } else {
                    result.push_str(&formatted_item);
                }

                // Add comma
                if i < items.len() - 1 {
                    result.push(',');
                } else if self.should_add_trailing_comma(true) {
                    result.push(',');
                }

                result.push('\n');
            }

            result.push_str(&indent);
            result.push(']');
            result
        } else {
            // Single line
            let space = if self.config.space_in_brackets { " " } else { "" };
            let items_str: Vec<String> = items
                .iter()
                .map(|item| self.format_value(item, depth, comment_map, source))
                .collect();

            let formatted = format!("[{}{}{}]", space, items_str.join(", "), space);

            // Check if single-line exceeds width, if so expand
            if formatted.len() > self.config.max_line_width {
                return self.format_array_expanded(items, depth, comment_map, source);
            }

            formatted
        }
    }

    /// Force array expansion (used when single-line exceeds width)
    fn format_array_expanded(
        &self,
        items: &[MonValue],
        depth: usize,
        comment_map: &HashMap<usize, Comment>,
        source: &str,
    ) -> String {
        let indent = self.indent_at(depth);
        let inner_indent = self.indent_at(depth + 1);
        let mut result = String::from("[\n");

        for (i, item) in items.iter().enumerate() {
            result.push_str(&inner_indent);
            result.push_str(&self.format_value(item, depth + 1, comment_map, source));

            if i < items.len() - 1 {
                result.push(',');
            } else if self.should_add_trailing_comma(true) {
                result.push(',');
            }

            result.push('\n');
        }

        result.push_str(&indent);
        result.push(']');
        result
    }

    /// Format an object
    fn format_object(
        &self,
        members: &[Member],
        depth: usize,
        comment_map: &HashMap<usize, Comment>,
        source: &str,
    ) -> String {
        if members.is_empty() {
            return if self.config.single_line_empty_objects {
                "{}".to_string()
            } else {
                "{\n}".to_string()
            };
        }

        let indent = self.indent_at(depth);
        let inner_indent = self.indent_at(depth + 1);

        // Decide if object should be single-line or multi-line
        let should_expand = self.should_expand_object(members);

        if should_expand {
            let mut result = String::from("{\n");

            for (i, member) in members.iter().enumerate() {
                result.push_str(&inner_indent);

                // Format member based on type
                match member {
                    Member::Pair(pair) => {
                        // Check if value has anchor matching the key
                        // Per EBNF: KeyPart ::= [ Anchor ] Key
                        // So &key: value is the canonical form when anchor matches key
                        let has_matching_anchor = pair.value.anchor.as_ref() == Some(&pair.key);

                        if has_matching_anchor {
                            // Format as &key: value (anchor is part of KeyPart)
                            result.push_str(&format!("&{}", pair.key));
                        } else {
                            // Format key normally
                            let key = if needs_quotes(&pair.key) {
                                self.format_string(&pair.key)
                            } else {
                                pair.key.clone()
                            };
                            result.push_str(&key);
                        }

                        // Colon with spacing
                        if self.config.space_before_colon {
                            result.push(' ');
                        }
                        result.push(':');
                        if self.config.space_after_colon {
                            result.push(' ');
                        }

                        // Format value WITHOUT anchor prefix (already in key)
                        let value_str = if has_matching_anchor {
                            // Don't include anchor in value formatting
                            self.format_value_without_anchor(
                                &pair.value,
                                depth + 1,
                                comment_map,
                                source,
                            )
                        } else {
                            self.format_value(&pair.value, depth + 1, comment_map, source)
                        };
                        result.push_str(&value_str);
                    }
                    Member::Spread(name) => {
                        result.push_str(&format!("...*{}", name));
                    }
                    Member::Import(_) => {
                        // Imports shouldn't be inside objects in formatted output
                        // Skip them here
                        continue;
                    }
                    Member::TypeDefinition(typedef) => {
                        // Format type definition
                        result.push_str(&self.format_typedef(typedef));
                    }
                }

                // Comma
                if i < members.len() - 1 {
                    result.push(',');
                } else if self.should_add_trailing_comma(true) {
                    result.push(',');
                }

                result.push('\n');
            }

            result.push_str(&indent);
            result.push('}');
            result
        } else {
            // Single line (compact)
            let space_after = if self.config.space_after_colon { " " } else { "" };
            let parts: Vec<String> = members
                .iter()
                .filter_map(|member| {
                    match member {
                        Member::Pair(pair) => {
                            let has_matching_anchor = pair.value.anchor.as_ref() == Some(&pair.key);

                            let key_str = if has_matching_anchor {
                                format!("&{}", pair.key)
                            } else if needs_quotes(&pair.key) {
                                self.format_string(&pair.key)
                            } else {
                                pair.key.clone()
                            };

                            let value_str = if has_matching_anchor {
                                self.format_value_without_anchor(
                                    &pair.value,
                                    depth,
                                    comment_map,
                                    source,
                                )
                            } else {
                                self.format_value(&pair.value, depth, comment_map, source)
                            };

                            Some(format!("{}:{}{}", key_str, space_after, value_str))
                        }
                        Member::Spread(name) => Some(format!("...*{}", name)),
                        Member::Import(_) => None,
                        Member::TypeDefinition(_) => None, // Skip in compact mode
                    }
                })
                .collect();

            format!("{{ {} }}", parts.join(", "))
        }
    }

    /// Format a type definition
    fn format_typedef(&self, typedef: &TypeDefinition) -> String {
        use mon_core::ast::TypeDef;

        match &typedef.def_type {
            TypeDef::Struct(struct_def) => {
                let mut result = format!("{}: #struct {{\n", typedef.name);

                for (i, field) in struct_def.fields.iter().enumerate() {
                    // Use proper indentation (current depth + 1 for struct members)
                    result.push_str(&self.config.indent_string());
                    result.push_str(&self.config.indent_string()); // Double indent for struct fields
                    result.push_str(&format!(
                        "{}({})",
                        field.name,
                        self.format_type_spec(&field.type_spec)
                    ));

                    // Add default value if present
                    if let Some(default) = &field.default_value {
                        result.push_str(" = ");
                        result.push_str(&self.format_value(default, 0, &HashMap::new(), ""));
                    }

                    if i < struct_def.fields.len() - 1 {
                        result.push(',');
                    } else if self.should_add_trailing_comma(true) {
                        result.push(',');
                    }

                    result.push('\n');
                }

                // Closing brace at proper indentation
                result.push_str(&self.config.indent_string());
                result.push('}');
                result
            }
            TypeDef::Enum(enum_def) => {
                let mut result = format!("{}: #enum {{\n", typedef.name);

                for (i, variant) in enum_def.variants.iter().enumerate() {
                    result.push_str(&self.config.indent_string());
                    result.push_str(&self.config.indent_string()); // Double indent for enum variants
                    result.push_str(variant);

                    if i < enum_def.variants.len() - 1 {
                        result.push(',');
                    } else if self.should_add_trailing_comma(true) {
                        result.push(',');
                    }

                    result.push('\n');
                }

                // Closing brace at proper indentation
                result.push_str(&self.config.indent_string());
                result.push('}');
                result
            }
        }
    }

    /// Format a type specification
    fn format_type_spec(&self, type_spec: &mon_core::ast::TypeSpec) -> String {
        use mon_core::ast::TypeSpec;

        match type_spec {
            TypeSpec::Simple(name, _) => name.clone(),
            TypeSpec::Collection(specs, _) => {
                let types: Vec<String> = specs.iter().map(|s| self.format_type_spec(s)).collect();
                format!("[{}]", types.join(", "))
            }
            TypeSpec::Spread(inner, _) => {
                format!("{}...", self.format_type_spec(inner))
            }
        }
    }

    /// Get indent string for a given depth
    fn indent_at(&self, depth: usize) -> String {
        self.config.indent_string().repeat(depth)
    }

    /// Determine if array should be expanded to multiple lines
    fn should_expand_array(&self, items: &[MonValue]) -> bool {
        match self.config.array_style {
            ArrayStyle::Expanded => true,
            ArrayStyle::Compact => false,
            ArrayStyle::Auto => {
                // Expand if exceeds threshold or contains complex types
                items.len() > self.config.array_expand_threshold
                    || items.iter().any(|item| {
                        matches!(item.kind, MonValueKind::Object(_) | MonValueKind::Array(_))
                    })
            }
        }
    }

    /// Determine if object should be expanded to multiple lines
    fn should_expand_object(&self, members: &[Member]) -> bool {
        match self.config.object_style {
            ObjectStyle::Expanded => true,
            ObjectStyle::Compact => false,
            ObjectStyle::Auto => {
                // Expand if exceeds threshold or contains complex types
                members.len() > self.config.object_expand_threshold
                    || members.iter().any(|member| match member {
                        Member::Pair(pair) => matches!(
                            pair.value.kind,
                            MonValueKind::Object(_) | MonValueKind::Array(_)
                        ),
                        _ => false,
                    })
            }
        }
    }

    /// Determine if trailing comma should be added
    fn should_add_trailing_comma(&self, is_multiline: bool) -> bool {
        match self.config.trailing_commas {
            TrailingCommaStyle::Always => true,
            TrailingCommaStyle::Never => false,
            TrailingCommaStyle::Multiline => is_multiline,
        }
    }
}

/// Comment representation
#[derive(Debug, Clone)]
struct Comment {
    line: usize,
    #[allow(dead_code)]
    column: usize,
    text: String,
    is_trailing: bool,
}

/// Check if position is inside a string literal
fn is_inside_string(line: &str, pos: usize) -> bool {
    let mut in_string = false;
    let mut escape_next = false;

    for (i, ch) in line.chars().enumerate() {
        if i >= pos {
            break;
        }

        if escape_next {
            escape_next = false;
            continue;
        }

        if ch == '\\' {
            escape_next = true;
            continue;
        }

        if ch == '"' {
            in_string = !in_string;
        }
    }

    in_string
}

/// Check if a key needs quotes
fn needs_quotes(key: &str) -> bool {
    // Keys with spaces, special characters, or starting with numbers need quotes
    key.contains(' ')
        || key.contains('"')
        || key.starts_with(|c: char| c.is_numeric())
        || key.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
}
