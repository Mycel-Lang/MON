# Changelog

All notable changes to the MON CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1] - 2024-11-23

### Initial Release

This is the first public release of the MON CLI—the official command-line interface for Mycel Object Notation.

### Added

#### Core Commands

- **`mon check`**: Validate MON file syntax and run linter

  - Position-aware diagnostics with precise error reporting
  - Symbol table for tracking anchors, types, and imports
  - Advanced linting rules (unused anchors, duplicate keys, etc.)
  - JSON output support for CI/CD integration

- **`mon fmt`**: Opinionated code formatter

  - Configurable via `.moncfg.mon`
  - Support for `--write`, `--check`, and `--watch` modes
  - Customizable indentation, line width, and quote styles
  - Advanced features: key sorting, comment alignment, smart line wrapping

- **`mon compile`**: Universal format converter

  - Export to JSON, YAML, and TOML
  - Handle null values in TOML with `--toml-null-value`
  - Generate comprehensive documentation and schemas with `--generate-docs`

- **`mon init`**: Project initialization

  - Interactive setup for configuration files
  - Generate unified `.moncfg.mon` with linter and formatter settings
  - Multiple templates: default, strict, lenient

- **`mon completions`**: Shell integration
  - Generate tab completions for Bash, Zsh, Fish, PowerShell, and Elvish
  - Context-aware autocomplete for commands and flags

#### Configuration

- Unified `.moncfg.mon` configuration file for both linter and formatter
- Support for multiple configuration templates
- Auto-formatting for generated config files

#### Licensing

- AGPLv3 dual-licensing model with open-source fork exception
- Contributor License Agreement (CLA) for streamlined contributions
- Commercial licensing available for proprietary use

### Known Limitations

- Some advanced features from the specification are still in development
- Performance optimizations ongoing for large files (>1MB)
- GitHub repository and additional distribution channels coming soon

### Dependencies

- Built on `mon-core` for parsing and resolution
- Requires Rust 2024 edition

---

[0.0.1]: https://crates.io/crates/mon/0.0.1
