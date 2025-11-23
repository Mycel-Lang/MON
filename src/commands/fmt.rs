use crate::formatter::{FormatConfig, Formatter, Style};
use colored::*;
use miette::Result;
use std::fs;

pub fn run(
    file: String,
    check: bool,
    write: bool,
    config: Option<String>,
    style: Option<String>,
) -> Result<()> {
    // Load configuration
    let config = if let Some(config_file) = config {
        println!("Loading config from: {}", config_file);
        FormatConfig::from_mon_file(&config_file)?
    } else if let Some(style) = style {
        match Style::from_str(&style) {
            Some(s) => {
                println!("Using {} style", style);
                s.to_config()
            }
            None => {
                return Err(miette::miette!(
                    "Unknown style: {}. Available styles: google, mozilla, airbnb, linux, rust, prettier, default",
                    style
                ));
            }
        }
    } else {
        // Try to find .monconfig.mon in current directory
        if std::path::Path::new(".monconfig.mon").exists() {
            println!("Found .monconfig.mon, using it");
            FormatConfig::from_mon_file(".monconfig.mon")?
        } else {
            FormatConfig::default()
        }
    };

    // Read source file
    let source = fs::read_to_string(&file)
        .map_err(|e| miette::miette!("Failed to read file {}: {}", file, e))?;

    // Format the file
    let formatter = Formatter::new(config);
    let formatted = formatter.format(&source)?;

    if check {
        // Check mode: verify if file is already formatted
        if source == formatted {
            println!("{} {}", "✓".green(), format!("{} is formatted correctly", file).bold());
            Ok(())
        } else {
            eprintln!("{} {}", "✗".red(), format!("{} needs formatting", file).bold());
            std::process::exit(1);
        }
    } else if write {
        // Write mode: write formatted output back to file
        fs::write(&file, &formatted)
            .map_err(|e| miette::miette!("Failed to write file {}: {}", file, e))?;
        println!("{} {}", "✓".green(), format!("Formatted {}", file).bold());
        Ok(())
    } else {
        // Default: print to stdout
        print!("{}", formatted);
        Ok(())
    }
}
