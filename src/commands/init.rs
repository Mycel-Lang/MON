// Interactive configuration generator

use console::{Term, style};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    Linter,
    Formatter,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub enum Template {
    Strict,
    Lenient,
    Default,
}

pub fn run(template: Option<Template>, interactive: bool, config_type: ConfigType) -> Result<()> {
    if interactive {
        run_interactive(config_type)
    } else {
        create_from_template(template.unwrap_or(Template::Default), config_type)
    }
}

fn run_interactive(initial_type: ConfigType) -> Result<()> {
    let term = Term::stdout();
    term.clear_screen().into_diagnostic()?;

    println!("{}", style("Welcome to MON Configuration!").cyan().bold());
    println!();

    // Ask what to configure
    let type_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to configure?")
        .items(&["Linter (.monlint.mon)", "Formatter  (.monfmt.mon)", "Both"])
        .default(match initial_type {
            ConfigType::Linter => 0,
            ConfigType::Formatter => 1,
            ConfigType::Both => 2,
        })
        .interact()
        .into_diagnostic()?;

    let config_type = match type_choice {
        0 => ConfigType::Linter,
        1 => ConfigType::Formatter,
        2 => ConfigType::Both,
        _ => unreachable!(),
    };

    match config_type {
        ConfigType::Linter => {
            let linter_config = create_linter_config_interactive()?;
            save_unified_config(Some(&linter_config), None)?;
        }
        ConfigType::Formatter => {
            let formatter_config = create_formatter_config_interactive()?;
            save_unified_config(None, Some(&formatter_config))?;
        }
        ConfigType::Both => {
            let linter_config = create_linter_config_interactive()?;
            println!();
            let formatter_config = create_formatter_config_interactive()?;
            save_unified_config(Some(&linter_config), Some(&formatter_config))?;
        }
    }

    Ok(())
}

fn create_linter_config_interactive() -> Result<String> {
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", style("Linter Configuration").cyan().bold());
    println!("{}", style("═".repeat(60)).dim());
    println!();

    // Complexity Limits
    println!("{}", style("COMPLEXITY LIMITS").yellow().bold());
    println!("{}", style("─".repeat(60)).dim());

    let max_nesting: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum nesting depth (3-10)")
        .default(5)
        .validate_with(|input: &u32| -> Result<(), String> {
            if (3..=10).contains(input) {
                Ok(())
            } else {
                Err("Value must be between 3 and 10".to_string())
            }
        })
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  Controls how deep your objects can be nested");
    println!("  ℹ️  Lower = stricter, easier to read | Higher = more flexible");
    println!();

    let max_object_members: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum object members (5-100)")
        .default(20)
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  How many fields an object can have");
    println!("  ℹ️  Helps keep objects focused and maintainable");
    println!();

    let max_array_elements: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum array elements  (5-200)")
        .default(50)
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  Maximum size for arrays");
    println!();

    // Warnings
    println!("{}", style("WARNINGS").yellow().bold());
    println!("{}", style("─".repeat(60)).dim());

    let warn_unused_anchors = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Warn about unused anchors?")
        .default(true)
        .interact()
        .into_diagnostic()?;

    println!("  ℹ️  Detects anchors (&name) that are never referenced (*name)");
    println!();

    let warn_magic_numbers = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Warn about magic numbers?")
        .default(false)
        .interact()
        .into_diagnostic()?;

    println!("  ℹ️  Magic numbers are hardcoded values without context");
    println!("  ℹ️  Example: timeout: 3600 (what does 3600 mean?)");
    println!();

    let warn_empty_structures = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Warn about empty objects/arrays?")
        .default(true)
        .interact()
        .into_diagnostic()?;

    println!("  ℹ️  Flags empty {{}} and [] to ensure they're intentional");
    println!();

    // Best Practices
    println!("{}", style("BEST PRACTICES").yellow().bold());
    println!("{}", style("─".repeat(60)).dim());

    let suggest_type_validation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Suggest type validation?")
        .default(true)
        .interact()
        .into_diagnostic()?;

    println!("  ℹ️  Recommends using :: Type for complex objects");
    println!();

    let enforce_consistent_naming = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enforce consistent naming style?")
        .default(false)
        .interact()
        .into_diagnostic()?;

    println!("  ℹ️  Warns if mixing snake_case and camelCase in same object");
    println!();

    // Advanced
    println!("{}", style("ADVANCED").yellow().bold());
    println!("{}", style("─".repeat(60)).dim());

    let max_spreads: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum spread operations (1-10)")
        .default(3)
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  Limits ...*spread usage in objects");
    println!();

    let max_import_depth: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum import chain depth (2-10)")
        .default(5)
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  Controls complexity of import dependencies");
    println!();

    // Generate config
    let config = format!(
        r#"import {{ LintConfig }} from "mon:types/linter"

{{
    // ═══════════════════════════════════════════════════
    // COMPLEXITY LIMITS
    // ═══════════════════════════════════════════════════
    
    /// Maximum nesting depth (3-10, default: 5)
    /// Controls LINT1001: Deep nesting warning
    max_nesting_depth: {},
    
    /// Maximum object members (5-100, default: 20)
    /// Controls LINT1002: Large object warning
    max_object_members: {},
    
    /// Maximum array elements (5-200, default: 50)
    /// Controls LINT1003: Large array warning
    max_array_elements: {},
    
    // ═══════════════════════════════════════════════════
    // CODE QUALITY
    // ═══════════════════════════════════════════════════
    
    /// Warn about unused anchors (default: true)
    /// Controls LINT2001: Unused anchor detection
    warn_unused_anchors: {},
    
    /// Warn about magic numbers (default: false)
    /// Controls LINT2004: Magic number detection
    warn_magic_numbers: {},
    
    /// Warn about empty structures (default: true)
    /// Controls LINT3003: Empty structures
    warn_empty_structures: {},
    
    /// Maximum spread operations (1-10, default: 3)
    /// Controls LINT2003: Excessive spreads
    max_spreads: {},
    
    // ═══════════════════════════════════════════════════
    // BEST PRACTICES
    // ═══════════════════════════════════════════════════
    
    /// Suggest type validation (default: true)
    /// Controls LINT3001: Missing type validation
    suggest_type_validation: {},
    
    /// Enforce consistent naming (default: false)
    /// Controls LINT3002: Inconsistent naming
    enforce_consistent_naming: {},
    
    /// Maximum import chain depth (2-10, default: 5)
    /// Controls LINT4001: Deep import chains
    max_import_chain_depth: {},
}}
"#,
        max_nesting,
        max_object_members,
        max_array_elements,
        warn_unused_anchors,
        warn_magic_numbers,
        warn_empty_structures,
        max_spreads,
        suggest_type_validation,
        enforce_consistent_naming,
        max_import_depth
    );

    // Show preview
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", style("Linter Configuration").cyan().bold());
    println!("{}", style("═".repeat(60)).dim());
    println!();
    println!("{}", style(&config).dim());

    Ok(config)
}

fn create_formatter_config_interactive() -> Result<String> {
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", style("Formatter Configuration").cyan().bold());
    println!("{}", style("═".repeat(60)).dim());
    println!();

    // Basic Options
    println!("{}", style("FORMATTING STYLE").yellow().bold());
    println!("{}", style("─".repeat(60)).dim());

    let indent_size: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Indentation size (2, 4, or 8)")
        .default(4)
        .validate_with(|input: &u32| -> Result<(), String> {
            if [2, 4, 8].contains(input) {
                Ok(())
            } else {
                Err("Value must be 2, 4, or 8".to_string())
            }
        })
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  Number of spaces for each indentation level");
    println!();

    let line_width: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum line width (40-120)")
        .default(80)
        .interact_text()
        .into_diagnostic()?;

    println!("  ℹ️  Lines longer than this will be wrapped");
    println!();

    let trailing_comma = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use trailing commas?")
        .default(true)
        .interact()
        .into_diagnostic()?;

    println!("  ℹ️  Add commas after the last item in arrays/objects");
    println!();

    let quote_style = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quote style")
        .items(&["double (\")", "single (')", "preserve"])
        .default(0)
        .interact()
        .into_diagnostic()?;

    println!();

    // Advanced Options
    println!("{}", style("ADVANCED").yellow().bold());
    println!("{}", style("─".repeat(60)).dim());

    let align_comments = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Align trailing comments?")
        .default(false)
        .interact()
        .into_diagnostic()?;

    let comment_column = if align_comments {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("  Comment alignment column (1-120)")
            .default(40)
            .interact_text()
            .into_diagnostic()?
    } else {
        40
    };

    println!();

    let sort_keys = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Sort object keys?")
        .items(&["none", "alphabetically", "by length"])
        .default(0)
        .interact()
        .into_diagnostic()?;

    println!();

    // Generate config
    let quote_str = match quote_style {
        0 => "double",
        1 => "single",
        2 => "preserve",
        _ => "double",
    };

    let sort_str = match sort_keys {
        0 => "none",
        1 => "alpha",
        2 => "length",
        _ => "none",
    };

    let config = format!(
        r#"import {{ FormatConfig }} from "mon:types/formatter"

{{
    /// Indentation size in spaces (2, 4, or 8; default: 4)
    indent_size: {},
    
    /// Maximum line width (40-120, default: 80)
    line_width: {},
    
    /// Use trailing commas (default: true)
    trailing_comma: {},
    
    /// Quote style: "double", "single", or "preserve"
    quote_style: "{}",
    
    /// Align trailing comments (default: false)
    align_trailing_comments: {},
    
    /// Column to align comments (1-120, default: 40)
    comment_alignment_column: {},
    
    /// Sort keys: "none", "alpha", or "length"
    sort_keys: "{}",
}}
"#,
        indent_size,
        line_width,
        trailing_comma,
        quote_str,
        align_comments,
        comment_column,
        sort_str
    );

    // Show preview
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", style("Formatter Configuration").cyan().bold());
    println!("{}", style("═".repeat(60)).dim());
    println!();
    println!("{}", style(&config).dim());

    Ok(config)
}

fn save_unified_config(linter: Option<&str>, formatter: Option<&str>) -> Result<()> {
    let config = match (linter, formatter) {
        (Some(l), Some(f)) => {
            // Both configs
            format!(
                r#"import {{ LintConfig }} from "mon:types/linter"
import {{ FormatConfig }} from "mon:types/formatter"

{{
    linter :: LintConfig = {}
    
    formatter :: FormatConfig = {}
}}
"#,
                l.trim_start_matches("import { LintConfig } from \"mon:types/linter\"\n\n"),
                f.trim_start_matches("import { FormatConfig } from \"mon:types/formatter\"\n\n")
            )
        }
        (Some(l), None) => {
            // Linter only
            l.to_string()
        }
        (None, Some(f)) => {
            // Formatter only
            f.to_string()
        }
        (None, None) => {
            return Ok(()); // Nothing to save
        }
    };

    // Show unified preview
    println!();
    println!("{}", style("═".repeat(60)).dim());
    println!("{}", style("Final Configuration").cyan().bold());
    println!("{}", style("═".repeat(60)).dim());
    println!();
    println!("{}", style(&config).dim());

    // Confirm save
    let should_save = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Save to .moncfg.mon?")
        .default(true)
        .interact()
        .into_diagnostic()?;

    if should_save {
        fs::write(".moncfg.mon", config).into_diagnostic()?;

        // Auto-format the generated file
        println!();
        println!("{}", style("⚙ Formatting .moncfg.mon...").cyan());

        let content = fs::read_to_string(".moncfg.mon").into_diagnostic()?;
        match crate::formatter::Formatter::new(crate::formatter::FormatConfig::default())
            .format(&content)
        {
            Ok(formatted) => {
                fs::write(".moncfg.mon", formatted).into_diagnostic()?;
                println!("{} Formatted successfully", style("✓").green());
            }
            Err(e) => {
                println!("{} Formatting skipped: {}", style("⚠").yellow(), e);
            }
        }

        println!();
        println!("{} Created .moncfg.mon", style("✓").green().bold());
        println!("{} Run 'mon check file.mon --lint' to test linter", style("ℹ️").cyan());
        println!("{} Run 'mon fmt file.mon' to test formatter", style("ℹ️").cyan());
    } else {
        println!();
        println!("{} Configuration not saved", style("✗").red());
    }

    Ok(())
}

fn create_from_template(template: Template, config_type: ConfigType) -> Result<()> {
    let content = match template {
        Template::Strict => include_str!("../../templates/unified-strict.mon"),
        Template::Lenient => include_str!("../../templates/unified-lenient.mon"),
        Template::Default => include_str!("../../templates/unified-default.mon"),
    };

    if Path::new(".moncfg.mon").exists() {
        println!("{} .moncfg.mon already exists", style("⚠").yellow());
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Overwrite?")
            .default(false)
            .interact()
            .into_diagnostic()?;

        if !overwrite {
            return Ok(());
        }
    }

    fs::write(".moncfg.mon", content).into_diagnostic()?;

    // Auto-format the generated file
    println!("{}", style("⚙ Formatting .moncfg.mon...").cyan());
    let content = fs::read_to_string(".moncfg.mon").into_diagnostic()?;
    match crate::formatter::Formatter::new(crate::formatter::FormatConfig::default())
        .format(&content)
    {
        Ok(formatted) => {
            fs::write(".moncfg.mon", formatted).into_diagnostic()?;
            println!("{} Formatted successfully", style("✓").green());
        }
        Err(e) => {
            println!("{} Formatting skipped: {}", style("⚠").yellow(), e);
        }
    }

    println!("{} Created .moncfg.mon ({:?} template)", style("✓").green(), template);

    Ok(())
}
