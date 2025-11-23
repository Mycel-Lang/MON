# MON CLI

[![Crates.io](https://img.shields.io/crates/v/mon.svg)](https://crates.io/crates/mon-tools)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](./LICENSE)

The official command line interface for Mycel Object Notation (MON).

Mycel Object Notation is a modern, type-safe configuration language designed for simplicity and correctness. The MON CLI provides a complete toolchain for working with MON files, including a high-performance parser, a configurable linter, an opinionated formatter, and a multi-format compiler.

For the full language specification and documentation, visit [mycel-lang.org](https://mycel-lang.org).

## Features

The MON CLI is a comprehensive toolkit for developers and operations teams managing configuration at scale.

### 1. Static Analysis and Linting

Ensure configuration correctness before deployment. The `mon check` command performs deep static analysis to catch errors early.

- **Syntax Validation**: Detects syntax errors with precise location reporting.
- **Type Checking**: Validates value types against expected schemas.
- **Code Quality Checks**: Identifies unused anchors, duplicate keys, and magic numbers.
- **Complexity Analysis**: Warns about deeply nested structures and large arrays or objects.
- **Best Practices**: Enforces consistent naming conventions and safe import chains.

### 2. Opinionated Formatter

Maintain a consistent code style across your entire project with `mon fmt`.

- **Zero Configuration Defaults**: Works out of the box with sensible defaults.
- **Fully Configurable**: Customize indentation, line width, quote style, and trailing commas via `.moncfg.mon`.
- **Advanced Formatting**: Supports key sorting, comment alignment, and smart line wrapping.
- **Watch Mode**: Automatically formats files as you save them for instant feedback.

### 3. Universal Compiler

Integrate MON into any ecosystem. The `mon compile` command converts MON files into standard formats consumed by existing applications.

- **JSON**: Generate standard JSON for web APIs and JavaScript applications.
- **YAML**: Output YAML for Kubernetes, Ansible, and CI/CD pipelines.
- **TOML**: Create TOML files for Rust and Python projects.
- **JSON Schema**: Automatically generate JSON Schemas from your MON definitions.

### 4. Project Initialization

Get started quickly with `mon init`.

- **Interactive Setup**: Guided prompts to create configuration files.
- **Unified Configuration**: Generates a single `.moncfg.mon` file for both linter and formatter settings.
- **Templates**: Choose from strict, lenient, or default configuration templates.

### 5. Shell Integration

Boost productivity with shell autocompletion.

- **Tab Completion**: Generate completion scripts for Bash, Zsh, Fish, PowerShell, and Elvish.
- **Context Aware**: Autocompletes commands, flags, and file paths.

## Installation

Install from crates.io:

```bash
cargo install mon
```

Or build from source:

```bash
cargo install --path .
```

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPLv3)** with a special exception for open-source forks.

**In short:**

- ✅ Free and open-source forever
- ✅ Open-source forks can use MON freely
- ✅ Commercial licenses available for proprietary use

See [`LICENSE`](./LICENSE) for details and [`CONTRIBUTING.md`](./CONTRIBUTING.md) for our mission.

For more information, visit [mycel-lang.org](https://mycel-lang.org).

Validate a file and run the linter:

```bash
mon check config.mon --lint
```

Output results as JSON for CI integration:

```bash
mon check config.mon --lint --as-json
```

### Formatting Code

Format a file in place:

```bash
mon fmt config.mon --write
```

Check if a file is correctly formatted (useful for CI):

```bash
mon fmt config.mon --check
```

Watch a file and format on save:

```bash
mon fmt config.mon --watch
```

### Compiling to Other Formats

Compile MON to JSON:

```bash
mon compile config.mon --to json
```

Compile to YAML:

```bash
mon compile config.mon --to yaml
```

Compile to TOML (handling null values):

```bash
mon compile config.mon --to toml --toml-null-value "nil"
```

Generate comprehensive documentation and schemas:

```bash
mon compile config.mon --generate-docs --output-dir ./dist
```

### Configuration

Initialize a new configuration file:

```bash
mon init
```

Generate shell completions (example for Fish):

```bash
mon completions fish > ~/.config/fish/completions/mon.fish
```

## Configuration File

The CLI is configured via a `.moncfg.mon` file in your project root. It supports detailed configuration for both the linter and formatter.

Example `.moncfg.mon`:

```mon
import { LintConfig } from "mon:types/linter"
import { FormatConfig } from "mon:types/formatter"

{
    linter :: LintConfig = {
        max_nesting_depth: 5,
        warn_unused_anchors: true,
        warn_magic_numbers: false,
        suggest_type_validation: true
    },

    formatter :: FormatConfig = {
        indent_size: 4,
        line_width: 80,
        quote_style: "double",
        sort_keys: "alpha"
    }
}
```

## License

This project is licensed under the MIT License.

For more information, visit [mycel-lang.org](https://mycel-lang.org).
