// Linter tests - Using JSON output for robust testing

use super::*;
use mon_core::parser::Parser;

// Helper function to run linter and parse JSON-like output
fn lint_source(source: &str, config: LintConfig) -> LintResult {
    let mut parser = Parser::new(source).unwrap();
    let doc = parser.parse_document().unwrap();
    let linter = Linter::new(config);
    let result = linter.lint(&doc, source).unwrap();
    println!("{:#?}", result);
    result
}

// Helper to count diagnostics by code
fn count_by_code(result: &LintResult, code: DiagnosticCode) -> usize {
    result.diagnostics.iter().filter(|d| d.code == code).count()
}

#[test]
fn test_linter_deep_nesting() {
    let source = r#"{
        a: {
            b: {
                c: {
                    d: {
                        e: {
                            value: "deep"
                        }
                    }
                }
            }
        }
    }"#;

    let config = LintConfig { max_nesting_depth: 3, ..Default::default() };

    let result = lint_source(source, config);

    // Verify behavior: should warn about exceeding nesting depth
    assert!(result.has_issues(), "Should detect nesting issue");
    assert_eq!(
        count_by_code(&result, DiagnosticCode::MaxNestingDepth),
        1,
        "Should have 1 nesting warning"
    );
    assert!(!result.warnings().is_empty(), "Should have warnings");
}

#[test]
fn test_linter_large_object() {
    let mut source = String::from("{\n");
    for i in 0..25 {
        source.push_str(&format!("    key{}: {},\n", i, i));
    }
    source.push_str("}");

    let config = LintConfig { max_object_members: 20, ..Default::default() };

    let result = lint_source(&source, config);

    // Verify behavior: should warn about large object
    assert!(result.has_issues());
    assert_eq!(count_by_code(&result, DiagnosticCode::MaxObjectMembers), 1);
}

#[test]
fn test_large_array() {
    let mut source = String::from("{\n    items: [\n");
    for i in 0..150 {
        source.push_str(&format!("        {},\n", i));
    }
    source.push_str("    ]\n}");

    let config = LintConfig { max_array_items: 100, ..Default::default() };

    let result = lint_source(&source, config);

    // Verify behavior: should warn about large array
    assert!(result.has_issues());
    assert_eq!(count_by_code(&result, DiagnosticCode::MaxArrayItems), 1);
}

#[test]
fn test_linter_unused_anchor() {
    let source = r#"{
        &unused: { value: 1 },
        &used: { value: 2 },
        data: *used
    }"#;

    let config = LintConfig { warn_unused_anchors: true, ..Default::default() };

    let result = lint_source(source, config);

    // Verify behavior: should warn about unused anchor
    assert!(result.has_issues());
    assert_eq!(count_by_code(&result, DiagnosticCode::UnusedAnchor), 1);
}

#[test]
fn test_linter_duplicate_key() {
    let source = r#"{
        key: "value1",
        key: "value2"
    }"#;

    let result = lint_source(source, LintConfig::default());

    // Verify behavior: should error on duplicate key
    assert!(result.has_issues());
    assert_eq!(count_by_code(&result, DiagnosticCode::DuplicateKey), 1);
    assert!(!result.errors().is_empty(), "Should be an error");
}

#[test]
fn test_linter_excessive_spreads() {
    let source = r#"{
        &a: { x: 1 },
        &b: { y: 2 },
        &c: { z: 3 },
        &d: { w: 4 },
        result: {
            ...*a,
            ...*b,
            ...*c,
            ...*d
        }
    }"#;

    let result = lint_source(source, LintConfig::default());

    // Verify behavior: should warn about excessive spreads
    assert!(result.has_issues());
    assert_eq!(count_by_code(&result, DiagnosticCode::ExcessiveSpreads), 1);
}

#[test]
fn test_linter_magic_numbers() {
    let source = r#"{
        value: 42.7
    }"#;

    let config = LintConfig { warn_magic_numbers: true, ..Default::default() };

    let result = lint_source(source, config);

    // Verify behavior: should hint about magic number
    assert!(result.has_issues());
    assert_eq!(count_by_code(&result, DiagnosticCode::MagicNumber), 1);
}

#[test]
fn test_linter_no_issues() {
    let source = r#"{
        name: "test",
        value: 1
    }"#;

    let result = lint_source(source, LintConfig::default());

    // Verify behavior: clean file should have no issues
    assert!(!result.has_issues());
    assert_eq!(result.diagnostics.len(), 0);
}

#[test]
fn test_diagnostic_severity() {
    let source = r#"{
        key: "v1",
        key: "v2"
    }"#;

    let result = lint_source(source, LintConfig::default());

    // Verify behavior: duplicate key is an error
    assert_eq!(result.errors().len(), 1);
    assert_eq!(result.errors()[0].severity, DiagnosticSeverity::Error);
}

#[test]
fn test_diagnostic_code_display() {
    let source = r#"{
        key: "v1",
        key: "v2"
    }"#;

    let result = lint_source(source, LintConfig::default());

    // Verify behavior: error code is LINT2002
    assert_eq!(result.errors()[0].code.code(), "LINT2002");
}

#[test]
fn test_lint_result_filtering() {
    let source = r#"{
        &unused: { x: 1 },
        key: "v1",
        key: "v2"
    }"#;

    let config = LintConfig { warn_unused_anchors: true, ..Default::default() };

    let result = lint_source(source, config);

    // Verify behavior: should have 1 error and 1 warning
    assert_eq!(result.errors().len(), 1, "Should have 1 error");
    assert_eq!(result.warnings().len(), 1, "Should have 1 warning");
    assert_eq!(result.infos().len(), 0, "Should have 0 infos");
}
