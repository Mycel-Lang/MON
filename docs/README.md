# MON CLI Documentation

> Comprehensive documentation for the Mycel Object Notation command-line interface

## Quick Start

```bash
# Install
cargo install mon

# Validate a file
mon check config.mon

# Format code
mon fmt config.mon --write

# Compile to JSON
mon compile config.mon --to json
```

## Commands

The MON CLI provides a complete toolchain for working with MON files:

### Core Commands

| Command                             | Description                      | Documentation       |
| ----------------------------------- | -------------------------------- | ------------------- |
| [`mon check`](./cli/check.md)       | Validate syntax and type errors  | Complete            |
| [`mon fmt`](./cli/fmt.md)           | Format MON files                 | Complete            |
| [`mon compile`](./api/README.md)    | Convert to JSON/YAML/TOML/Schema | Needs update        |
| [`mon init`](./examples/configs.md) | Create configuration files       | Needs dedicated doc |

### Code Quality

| Command                     | Description                 | Documentation            |
| --------------------------- | --------------------------- | ------------------------ |
| [`mon lint`](./cli/lint.md) | Run linter for code quality | Complete (new in v0.0.1) |

### Project Management

| Command                         | Description                          | Documentation            |
| ------------------------------- | ------------------------------------ | ------------------------ |
| [`mon bundle`](./cli/bundle.md) | Resolve imports, create bundled file | Complete (new in v0.0.1) |

### Development Tools

| Command                                               | Description                | Documentation       |
| ----------------------------------------------------- | -------------------------- | ------------------- |
| [`mon completions`](./cli/check.md#shell-completions) | Generate shell completions | Needs dedicated doc |

## Feature Areas

### [Linter](./linter/README.md)

Comprehensive code quality analysis with configurable rules.

**Topics:**

- [Configuration](./linter/configuration.md)
- [Rules Reference](./linter/rules.md)
- [Best Practices](./linter/best-practices.md)
- [Complexity Analysis](./linter/complexity.md)
- [Code Smells](./linter/smells.md)
- [Import Analysis](./linter/imports.md)

### [Formatter](./formatter/configuration.md)

Opinionated code formatting with multiple style presets.

**Topics:**

- [Configuration](./formatter/configuration.md)

### [LSP Support](./lsp/README.md)

Language Server Protocol integration for IDE features (planned).

### [Examples](./examples/)

- [CLI Workflows](./examples/cli-workflow.md)
- [Configuration Examples](./examples/configs.md)

## Development Documentation

Internal development docs have been moved to [`docs/dev/`](./dev/):

- [Comprehensive Export](./dev/COMPREHENSIVE_EXPORT.md) - Export feature documentation
- [Formatter Enhancements](./dev/FORMATTER_ENHANCEMENTS.md) - Formatter improvements
- [Formatter Fixes](./dev/FORMATTER_FIXES.md) - Formatter bug fixes
- [Format Limitations](./dev/FORMAT_LIMITATIONS.md) - Known formatting limitations
- [MON Core API Analysis](./dev/MON_CORE_API_ANALYSIS.md) - Core library API details
- [MON Language Reference](./dev/MON_LANGUAGE_REFERENCE.md) - Language specification
- [API Specification](./dev/tempfile.md) - Complete CLI API spec

## Getting Help

```bash
# General help
mon --help

# Command-specific help
mon check --help
mon fmt --help
mon lint --help
mon bundle --help
```

## Version

Check your installed version:

```bash
mon --version
```

## Contributing

See [`CONTRIBUTING.md`](../CONTRIBUTING.md) for guidelines on contributing to MON CLI.

## License

MON CLI is licensed under **AGPLv3** with an open-source fork exception. See [`LICENSE`](../LICENSE) for details.
