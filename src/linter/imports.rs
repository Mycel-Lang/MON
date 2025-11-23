// Import analysis for MON files

use crate::linter::{LintConfig, LintResult, diagnostic::DiagnosticCode};
use mon_core::ast::{Member, MonValue, MonValueKind};
use std::collections::HashSet;

pub struct ImportAnalyzer {
    config: LintConfig,
}

impl ImportAnalyzer {
    pub fn new(config: LintConfig) -> Self {
        Self { config }
    }

    pub fn analyze(&self, root: &MonValue, result: &mut LintResult) {
        // Collect all used references (anchors, aliases, types)
        let used_refs = self.collect_used_references(root);

        // Note: To properly detect unused imports, we'd need access to MonDocument.imports
        // For now, we can detect deep import chains by counting namespace usage
        self.detect_namespace_complexity(root, result);
    }

    fn collect_used_references(&self, value: &MonValue) -> HashSet<String> {
        let mut used = HashSet::new();
        self.collect_used_recursive(value, &mut used);
        used
    }

    fn collect_used_recursive(&self, value: &MonValue, used: &mut HashSet<String>) {
        match &value.kind {
            MonValueKind::Alias(name) => {
                used.insert(name.clone());
            }
            MonValueKind::ArraySpread(name) => {
                used.insert(name.clone());
            }
            MonValueKind::Object(members) => {
                for member in members {
                    match member {
                        Member::Pair(pair) => {
                            self.collect_used_recursive(&pair.value, used);
                        }
                        Member::Spread(name) => {
                            used.insert(name.clone());
                        }
                        _ => {}
                    }
                }
            }
            MonValueKind::Array(items) => {
                for item in items {
                    self.collect_used_recursive(item, used);
                }
            }
            _ => {}
        }
    }

    fn detect_namespace_complexity(&self, value: &MonValue, result: &mut LintResult) {
        // Detect if there are too many namespaced references
        // This could indicate complex import chains
        let namespace_references = self.count_namespace_refs(value);

        if namespace_references > self.config.max_import_chain_depth * 3 {
            result.add_diagnostic(
                DiagnosticCode::DeepImportChain,
                format!(
                    "File has {} namespaced references, consider simplifying imports",
                    namespace_references
                ),
                None,
            );
        }
    }

    fn count_namespace_refs(&self, value: &MonValue) -> usize {
        let mut count = 0;
        self.count_namespace_refs_recursive(value, &mut count);
        count
    }

    fn count_namespace_refs_recursive(&self, value: &MonValue, count: &mut usize) {
        match &value.kind {
            MonValueKind::Alias(name) | MonValueKind::ArraySpread(name) => {
                // Namespaced references contain a dot
                if name.contains('.') {
                    *count += 1;
                }
            }
            MonValueKind::Object(members) => {
                for member in members {
                    match member {
                        Member::Pair(pair) => {
                            self.count_namespace_refs_recursive(&pair.value, count);
                        }
                        Member::Spread(name) => {
                            if name.contains('.') {
                                *count += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            MonValueKind::Array(items) => {
                for item in items {
                    self.count_namespace_refs_recursive(item, count);
                }
            }
            _ => {}
        }
    }
}
