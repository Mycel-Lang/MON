use crate::linter::{LintConfig, Linter};
use colored::*;
use miette::Result;
use mon_core::parser::Parser;
use std::fs;
use std::path::Path;

pub fn run(
    files: Vec<String>,
    fix: bool,
    config: Option<String>,
    format: String,
    rules: Option<Vec<String>>,
    no_rules: Option<Vec<String>>,
) -> Result<()> {
    // Load config
    let mut lint_config = if let Some(config_path) = config {
        load_config(&config_path)?
    } else {
        // Try to find .moncfg.mon in current directory
        if Path::new(".moncfg.mon").exists() {
            load_config(".moncfg.mon")?
        } else {
            LintConfig::default()
        }
    };

    // Apply rule filtering
    if let Some(enabled_rules) = rules {
        lint_config = lint_config.with_only_rules(enabled_rules);
    }
    if let Some(disabled_rules) = no_rules {
        lint_config = lint_config.without_rules(disabled_rules);
    }

    let linter = Linter::new(lint_config);
    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut all_results = Vec::new();

    for file in &files {
        let content = fs::read_to_string(file)
            .map_err(|e| miette::miette!("Failed to read file {}: {}", file, e))?;

        let mut parser = Parser::new(&content)
            .map_err(|e| miette::miette!("Parser initialization failed for {}: {:?}", file, e))?;

        let doc = parser
            .parse_document()
            .map_err(|e| miette::miette!("Parse error in {}: {:?}", file, e))?;

        let lint_result = linter.lint(&doc, &content)?;

        total_errors += lint_result.errors().len();
        total_warnings += lint_result.warnings().len();

        all_results.push((file.clone(), lint_result));
    }

    // Output results
    match format.as_str() {
        "json" => output_json(&all_results)?,
        "sarif" => output_sarif(&all_results)?,
        _ => output_text(&all_results, fix)?,
    }

    // Exit with error if there are errors
    if total_errors > 0 {
        std::process::exit(4); // Exit code 4 for lint errors (per spec)
    }

    Ok(())
}

fn load_config(path: &str) -> Result<LintConfig> {
    let content = fs::read_to_string(path)
        .map_err(|e| miette::miette!("Failed to read config file: {}", e))?;

    let mut parser =
        Parser::new(&content).map_err(|e| miette::miette!("Failed to parse config: {:?}", e))?;

    let doc =
        parser.parse_document().map_err(|e| miette::miette!("Config parse error: {:?}", e))?;

    LintConfig::from_document(&doc).map_err(|e| miette::miette!("Invalid lint config: {}", e))
}

fn output_text(results: &[(String, crate::linter::LintResult)], _fix: bool) -> Result<()> {
    for (file, lint_result) in results {
        if !lint_result.has_issues() {
            println!("{}", format!("  ✓ {}", file).green());
        } else {
            println!("\n{}", file.bold());

            let errors = lint_result.errors();
            let warnings = lint_result.warnings();
            let infos = lint_result.infos();

            for diag in errors {
                println!(
                    "  {} {} {}",
                    diag.severity.short_label().red().bold(),
                    diag.code.to_string().yellow(),
                    diag.message
                );
                if let Some(ref location_str) = diag.location {
                    println!("    at {}", location_str);
                } else if let Some(ref range) = diag.range {
                    println!("    at line {}", range.start.line);
                }
            }

            for diag in warnings {
                println!(
                    "  {} {} {}",
                    diag.severity.short_label().yellow().bold(),
                    diag.code.to_string().cyan(),
                    diag.message
                );
                if let Some(ref location_str) = diag.location {
                    println!("    at {}", location_str);
                } else if let Some(ref range) = diag.range {
                    println!("    at line {}", range.start.line);
                }
            }

            for diag in infos {
                println!(
                    "  {} {} {}",
                    diag.severity.short_label().blue().bold(),
                    diag.code.to_string().cyan(),
                    diag.message
                );
            }
        }
    }

    println!();
    let total_errors: usize = results.iter().map(|(_, r)| r.errors().len()).sum();
    let total_warnings: usize = results.iter().map(|(_, r)| r.warnings().len()).sum();

    println!("{}: {} files linted", "Summary".bold(), results.len());
    if total_errors > 0 {
        println!("  {} file(s) with errors", total_errors.to_string().red());
    }
    if total_warnings > 0 {
        println!("  {} file(s) with warnings", total_warnings.to_string().yellow());
    }

    Ok(())
}

fn output_json(results: &[(String, crate::linter::LintResult)]) -> Result<()> {
    let json = serde_json::to_string_pretty(results)
        .map_err(|e| miette::miette!("Failed to serialize results: {}", e))?;
    println!("{}", json);
    Ok(())
}

fn output_sarif(_results: &[(String, crate::linter::LintResult)]) -> Result<()> {
    // TODO: Implement SARIF format
    // For now, just return JSON
    Err(miette::miette!("SARIF output format not yet implemented"))
}
