// Complexity analysis for MON structures

use crate::linter::{LintConfig, LintResult, diagnostic::DiagnosticCode};
use mon_core::ast::{Member, MonValue, MonValueKind};

pub struct ComplexityAnalyzer {
    config: LintConfig,
}

impl ComplexityAnalyzer {
    pub fn new(config: LintConfig) -> Self {
        Self { config }
    }

    pub fn analyze(&self, root: &MonValue, result: &mut LintResult) {
        // Analyze nesting depth
        let max_depth = self.calculate_max_depth(root, 0);
        if max_depth > self.config.max_nesting_depth {
            if max_depth > self.config.max_nesting_depth + 2 {
                result.add_diagnostic(
                    DiagnosticCode::MaxNestingDepth,
                    format!(
                        "Maximum nesting depth of {} exceeds limit of {} by more than 2 levels",
                        max_depth, self.config.max_nesting_depth
                    ),
                    None,
                );
            } else {
                result.add_diagnostic(
                    DiagnosticCode::MaxNestingDepth,
                    format!(
                        "Maximum nesting depth of {} exceeds recommended limit of {}",
                        max_depth, self.config.max_nesting_depth
                    ),
                    None,
                );
            }
        }

        // Analyze object/array sizes
        self.analyze_sizes(root, result, 0);
    }

    fn calculate_max_depth(&self, value: &MonValue, current_depth: usize) -> usize {
        match &value.kind {
            MonValueKind::Object(members) => {
                let mut max = current_depth;
                for member in members {
                    match member {
                        Member::Pair(pair) => {
                            let depth = self.calculate_max_depth(&pair.value, current_depth + 1);
                            max = max.max(depth);
                        }
                        Member::TypeDefinition(_) => {}
                        Member::Spread(_) => {}
                        Member::Import(_) => {}
                    }
                }
                max
            }
            MonValueKind::Array(items) => {
                let mut max = current_depth;
                for item in items {
                    let depth = self.calculate_max_depth(item, current_depth + 1);
                    max = max.max(depth);
                }
                max
            }
            _ => current_depth,
        }
    }

    fn analyze_sizes(&self, value: &MonValue, result: &mut LintResult, depth: usize) {
        match &value.kind {
            MonValueKind::Object(members) => {
                // Count non-type-definition members
                let regular_members: Vec<_> =
                    members.iter().filter(|m| !matches!(m, Member::TypeDefinition(_))).collect();

                if regular_members.len() > self.config.max_object_members {
                    result.add_diagnostic(
                        DiagnosticCode::MaxObjectMembers,
                        format!(
                            "Object at depth {} has {} members, exceeds recommended limit of {}",
                            depth,
                            regular_members.len(),
                            self.config.max_object_members
                        ),
                        None,
                    );
                }

                // Recurse into nested structures
                for member in members {
                    if let Member::Pair(pair) = member {
                        self.analyze_sizes(&pair.value, result, depth + 1);
                    }
                }
            }
            MonValueKind::Array(items) => {
                if items.len() > self.config.max_array_items {
                    result.add_diagnostic(
                        DiagnosticCode::MaxArrayItems,
                        format!(
                            "Array at depth {} has {} items, exceeds recommended limit of {}",
                            depth,
                            items.len(),
                            self.config.max_array_items
                        ),
                        None,
                    );
                }

                // Recurse into nested structures
                for item in items {
                    self.analyze_sizes(item, result, depth + 1);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mon_core::parser::Parser;

    #[test]
    fn test_nesting_depth() {
        let source = r#"{
            a: {
                b: {
                    c: {
                        d: {
                            e: "deep"
                        }
                    }
                }
            }
        }"#;

        let mut parser = Parser::new(source).unwrap();
        let doc = parser.parse_document().unwrap();

        let config = LintConfig { max_nesting_depth: 3, ..Default::default() };

        let analyzer = ComplexityAnalyzer::new(config);
        let mut result = LintResult::new();
        analyzer.analyze(&doc.root, &mut result);

        assert!(!result.warnings().is_empty() || !result.errors().is_empty());
    }

    #[test]
    fn test_large_object() {
        let mut source = String::from("{\n");
        for i in 0..25 {
            source.push_str(&format!("    key{}: {},\n", i, i));
        }
        source.push_str("}");

        let mut parser = Parser::new(&source).unwrap();
        let doc = parser.parse_document().unwrap();

        let config = LintConfig { max_object_members: 20, ..Default::default() };

        let analyzer = ComplexityAnalyzer::new(config);
        let mut result = LintResult::new();
        analyzer.analyze(&doc.root, &mut result);

        assert!(!result.warnings().is_empty());
    }
}
