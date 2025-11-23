use clap::{Parser, Subcommand};
use clap_complete::Shell;
use miette::Result;

mod commands;
mod errors;
mod formatter;
mod linter;

#[derive(Parser)]
#[command(name = "mon")]
#[command(about = "Mycel Object Notation (MON) CLI Tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check a MON file for syntax and type errors
    Check {
        /// The MON file to check
        file: String,
        /// Run linter for code quality analysis
        #[arg(long)]
        lint: bool,
        /// Output results as JSON
        #[arg(long)]
        as_json: bool,
    },
    /// Format a MON file
    Fmt {
        /// The MON file to format
        file: String,
        /// Check if the file is formatted without modifying it
        #[arg(long, short)]
        check: bool,
        /// Write formatted output back to the file
        #[arg(long, short)]
        write: bool,
        /// Configuration file path
        #[arg(long)]
        config: Option<String>,
        /// Formatting style (google, mozilla, airbnb, linux, rust, prettier)
        #[arg(long)]
        style: Option<String>,
        /// Watch mode - continuously format on file changes
        #[arg(long)]
        watch: bool,
    },
    /// Compile a MON file to another format
    Compile {
        /// The MON file to compile
        file: String,
        /// The output format (json, yaml, toml, json-schema)
        #[arg(long, short, default_value = "json")]
        to: String,
        /// For TOML output: replace null values with this string (default: omit null fields)
        #[arg(long)]
        toml_null_value: Option<String>,
        /// Output directory for comprehensive export (includes schemas and data)
        #[arg(long, short)]
        output_dir: Option<String>,
        /// Generate comprehensive documentation (structure, stats, dependencies)
        #[arg(long)]
        generate_docs: bool,
    },
    /// Create MON configuration files interactively
    Init {
        /// Use predefined template: strict, lenient, or default
        #[arg(long, short)]
        template: Option<String>,
        /// Configuration type: linter, formatter, or both
        #[arg(long, default_value = "both")]
        config: String,
        /// Non-interactive mode (use with --template)
        #[arg(long)]
        non_interactive: bool,
    },
    /// Generate shell completion scripts
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Run linter for code quality analysis
    Lint {
        /// The MON files to lint
        files: Vec<String>,
        /// Auto-fix issues where possible
        #[arg(long, short)]
        fix: bool,
        /// Configuration file path
        #[arg(long)]
        config: Option<String>,
        /// Output format (text/json/sarif)
        #[arg(long, default_value = "text")]
        format: String,
        /// Comma-separated rule IDs to enable
        #[arg(long)]
        rules: Option<String>,
        /// Comma-separated rule IDs to disable
        #[arg(long)]
        no_rules: Option<String>,
    },
    /// Resolve imports and create a single bundled file
    Bundle {
        /// The entry MON file
        entry_file: String,
        /// Output file path (stdout if omitted)
        #[arg(long, short)]
        output: Option<String>,
        /// Output format (mon/json/yaml/toml)
        #[arg(long, short, default_value = "mon")]
        to: String,
        /// Minify output (remove comments and whitespace)
        #[arg(long, short)]
        minify: bool,
        /// Remove unused definitions
        #[arg(long)]
        tree_shake: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file, lint, as_json } => {
            commands::check::run(&file, lint, as_json)?;
            Ok(())
        }
        Commands::Fmt { file, check, write, config, style, watch } => {
            if watch {
                let paths = vec![std::path::PathBuf::from(&file)];
                let watch_mode = crate::formatter::WatchMode::new(paths, write);
                watch_mode.run()?;
            } else {
                commands::fmt::run(file, check, write, config, style)?;
            }
            Ok(())
        }
        Commands::Compile { file, to, toml_null_value, output_dir, generate_docs } => {
            commands::compile::run(file, to, toml_null_value, output_dir, generate_docs)
        }
        Commands::Init { template, config, non_interactive } => {
            let template_type = template.as_ref().map(|t| match t.as_str() {
                "strict" => commands::init::Template::Strict,
                "lenient" => commands::init::Template::Lenient,
                _ => commands::init::Template::Default,
            });

            let config_type = match config.as_str() {
                "linter" => commands::init::ConfigType::Linter,
                "formatter" => commands::init::ConfigType::Formatter,
                _ => commands::init::ConfigType::Both,
            };

            commands::init::run(template_type, !non_interactive, config_type)?;
            Ok(())
        }
        Commands::Completions { shell } => {
            commands::completions::run(shell)?;
            Ok(())
        }
        Commands::Lint { files, fix, config, format, rules, no_rules } => {
            let rules_vec = rules.map(|r| r.split(',').map(String::from).collect());
            let no_rules_vec = no_rules.map(|r| r.split(',').map(String::from).collect());
            commands::lint::run(files, fix, config, format, rules_vec, no_rules_vec)
        }
        Commands::Bundle { entry_file, output, to, minify, tree_shake } => {
            commands::bundle::run(entry_file, output, to, minify, tree_shake)
        }
    }
}
