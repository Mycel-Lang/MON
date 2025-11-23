use crate::linter::{LintConfig, Linter};
use colored::*;
use miette::Result;
use mon_core::parser::Parser;
use std::fs;

pub fn run(file: &str, lint: bool, as_json: bool) -> Result<()> {
    println!("Checking {}...", file);

    let content =
        fs::read_to_string(file).map_err(|e| miette::miette!("Failed to read file: {}", e))?;

    let mut parser = Parser::new(&content)
        .map_err(|e| miette::miette!("Parser initialization failed: {:?}", e))?;

    let doc = parser.parse_document().map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    // If lint flag is enabled, run linter
    if lint {
        let linter = Linter::new(LintConfig::default());
        let lint_result = linter.lint(&doc, &content)?;

        if as_json {
            // Output JSON
            let json = serde_json::to_string_pretty(&lint_result)
                .map_err(|e| miette::miette!("Failed to serialize lint results: {}", e))?;
            println!("{}", json);

            if !lint_result.errors().is_empty() {
                std::process::exit(1);
            }
        } else {
            // Human-readable output
            if !lint_result.has_issues() {
                println!("{}  {}", "Linting".green().bold(), file);
                println!();
                println!("  {} No issues found", "✓".green().bold());
            } else {
                println!("{}  {}", "Linting".green().bold(), file);
                println!();

                let errors = lint_result.errors();
                let warnings = lint_result.warnings();
                let infos = lint_result.infos();

                for diag in &errors {
                    println!(
                        "  {} {} {}",
                        diag.severity.short_label().red().bold(),
                        diag.code.to_string().yellow(),
                        diag.message
                    );
                }

                for diag in &warnings {
                    println!(
                        "  {} {} {}",
                        diag.severity.short_label().yellow().bold(),
                        diag.code.to_string().cyan(),
                        diag.message
                    );
                }

                for diag in &infos {
                    println!(
                        "  {} {} {}",
                        diag.severity.short_label().blue().bold(),
                        diag.code.to_string().cyan(),
                        diag.message
                    );
                }

                println!();
                println!(
                    "{}: {} errors, {} warnings, {} hints",
                    "Summary".bold(),
                    errors.len(),
                    warnings.len(),
                    infos.len()
                );

                if !errors.is_empty() || !warnings.is_empty() {
                    println!();
                    println!(
                        "{}",
                        "Run 'mon check --explain <CODE>' for detailed information".to_string()
                            .dimmed()
                    );
                    println!("{}", "Configure rules in .monlint.mon".to_string().dimmed());
                }

                if !errors.is_empty() {
                    return Err(miette::miette!("Linting found {} error(s)", errors.len()));
                }
            }
        }
    } else if as_json {
        // Just syntax check with JSON output
        println!("{{\"status\":\"ok\",\"message\":\"Syntax OK\"}}");
    } else {
        println!("{}", "Syntax OK".green().bold());
    }

    Ok(())
}
