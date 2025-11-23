// Code smell detection for MON files

use crate::linter::{LintResult, diagnostic::DiagnosticCode};
use mon_core::ast::{Member, MonValue, MonValueKind};
use std::collections::{HashMap, HashSet};

pub struct SmellDetector {
    warn_unused_anchors: bool,
    warn_magic_numbers: bool,
}

impl SmellDetector {
    pub fn new(warn_unused_anchors: bool, warn_magic_numbers: bool) -> Self {
        Self { warn_unused_anchors, warn_magic_numbers }
    }

    pub fn detect(&self, root: &MonValue, result: &mut LintResult) {
        if self.warn_unused_anchors {
            self.detect_unused_anchors(root, result);
        }

        if self.warn_magic_numbers {
            self.detect_magic_numbers(root, result);
        }

        self.detect_duplicate_keys(root, result);
        self.detect_excessive_spreads(root, result);
        self.detect_empty_structures(root, result);
        self.detect_inconsistent_naming(root, result);
    }

    fn detect_unused_anchors(&self, root: &MonValue, result: &mut LintResult) {
        let mut defined_anchors = HashSet::new();
        let mut used_anchors = HashSet::new();

        // Collect defined anchors
        self.collect_anchors(root, &mut defined_anchors);

        // Collect used aliases and spreads
        self.collect_aliases(root, &mut used_anchors);

        // Find unused
        for anchor in &defined_anchors {
            if !used_anchors.contains(anchor) {
                result.add_diagnostic(
                    DiagnosticCode::UnusedAnchor,
                    format!("Anchor '{}' is defined but never used", anchor),
                    None,
                );
            }
        }
    }

    fn collect_anchors(&self, value: &MonValue, anchors: &mut HashSet<String>) {
        if let Some(anchor) = &value.anchor {
            anchors.insert(anchor.clone());
        }

        match &value.kind {
            MonValueKind::Object(members) => {
                for member in members {
                    if let Member::Pair(pair) = member {
                        self.collect_anchors(&pair.value, anchors);
                    }
                }
            }
            MonValueKind::Array(items) => {
                for item in items {
                    self.collect_anchors(item, anchors);
                }
            }
            _ => {}
        }
    }

    fn collect_aliases(&self, value: &MonValue, aliases: &mut HashSet<String>) {
        match &value.kind {
            MonValueKind::Alias(name) => {
                aliases.insert(name.clone());
            }
            MonValueKind::ArraySpread(name) => {
                aliases.insert(name.clone());
            }
            MonValueKind::Object(members) => {
                for member in members {
                    match member {
                        Member::Pair(pair) => {
                            self.collect_aliases(&pair.value, aliases);
                        }
                        Member::Spread(name) => {
                            aliases.insert(name.clone());
                        }
                        _ => {}
                    }
                }
            }
            MonValueKind::Array(items) => {
                for item in items {
                    self.collect_aliases(item, aliases);
                }
            }
            _ => {}
        }
    }

    fn detect_magic_numbers(&self, value: &MonValue, result: &mut LintResult) {
        match &value.kind {
            MonValueKind::Number(n) => {
                // Skip common values (0, 1, -1, 100, 1000)
                let abs_n = n.abs();
                if abs_n > 1.0 && abs_n != 10.0 && abs_n != 100.0 && abs_n != 1000.0 {
                    result.add_diagnostic(
                        DiagnosticCode::MagicNumber,
                        format!("Consider extracting magic number {} into a named constant", n),
                        None,
                    );
                }
            }
            MonValueKind::Object(members) => {
                for member in members {
                    if let Member::Pair(pair) = member {
                        self.detect_magic_numbers(&pair.value, result);
                    }
                }
            }
            MonValueKind::Array(items) => {
                for item in items {
                    self.detect_magic_numbers(item, result);
                }
            }
            _ => {}
        }
    }

    fn detect_duplicate_keys(&self, value: &MonValue, result: &mut LintResult) {
        if let MonValueKind::Object(members) = &value.kind {
            let mut seen_keys = HashMap::new();

            for member in members {
                if let Member::Pair(pair) = member {
                    if seen_keys.insert(pair.key.clone(), ()).is_some() {
                        result.add_diagnostic(
                            DiagnosticCode::DuplicateKey,
                            format!("Duplicate key '{}' in object", pair.key),
                            None,
                        );
                    }

                    // Recurse
                    self.detect_duplicate_keys(&pair.value, result);
                }
            }
        } else if let MonValueKind::Array(items) = &value.kind {
            for item in items {
                self.detect_duplicate_keys(item, result);
            }
        }
    }

    fn detect_excessive_spreads(&self, value: &MonValue, result: &mut LintResult) {
        if let MonValueKind::Object(members) = &value.kind {
            let spread_count = members.iter().filter(|m| matches!(m, Member::Spread(_))).count();

            if spread_count > 3 {
                result.add_diagnostic(
                    DiagnosticCode::ExcessiveSpreads,
                    format!("Object has {} spreads, consider simplifying", spread_count),
                    None,
                );
            }

            // Recurse
            for member in members {
                if let Member::Pair(pair) = member {
                    self.detect_excessive_spreads(&pair.value, result);
                }
            }
        } else if let MonValueKind::Array(items) = &value.kind {
            for item in items {
                self.detect_excessive_spreads(item, result);
            }
        }
    }

    fn detect_empty_structures(&self, value: &MonValue, result: &mut LintResult) {
        match &value.kind {
            MonValueKind::Object(members) => {
                // Filter out type definitions
                let regular_members: Vec<_> =
                    members.iter().filter(|m| !matches!(m, Member::TypeDefinition(_))).collect();

                if regular_members.is_empty() {
                    result.add_diagnostic(
                        DiagnosticCode::EmptyObject,
                        "Empty object found - verify this is intentional".to_string(),
                        None,
                    );
                }

                // Recurse
                for member in members {
                    if let Member::Pair(pair) = member {
                        self.detect_empty_structures(&pair.value, result);
                    }
                }
            }
            MonValueKind::Array(items) => {
                if items.is_empty() {
                    result.add_diagnostic(
                        DiagnosticCode::EmptyObject,
                        "Empty array found - verify this is intentional".to_string(),
                        None,
                    );
                }

                for item in items {
                    self.detect_empty_structures(item, result);
                }
            }
            _ => {}
        }
    }

    fn detect_inconsistent_naming(&self, value: &MonValue, result: &mut LintResult) {
        if let MonValueKind::Object(members) = &value.kind {
            let mut snake_case_count = 0;
            let mut camel_case_count = 0;

            for member in members {
                if let Member::Pair(pair) = member {
                    let key = &pair.key;
                    if key.contains('_') {
                        snake_case_count += 1;
                    } else if key.chars().any(|c| c.is_uppercase()) {
                        camel_case_count += 1;
                    }

                    // Recurse
                    self.detect_inconsistent_naming(&pair.value, result);
                }
            }

            // If we have both styles in same object, warn
            if snake_case_count > 0 && camel_case_count > 0 {
                result.add_diagnostic(
                    DiagnosticCode::InconsistentNaming,
                    format!(
                        "Object has mixed naming styles ({} snake_case, {} camelCase)",
                        snake_case_count, camel_case_count
                    ),
                    None,
                );
            }
        }
    }
}
