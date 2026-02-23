<p align="center">
  <h1 align="center">LintScout</h1>
  <p align="center">
    <strong>Find every linter ignore directive hiding in your codebase.</strong>
  </p>
  <p align="center">
    <a href="https://github.com/eladbash/LintScout/actions/workflows/ci.yml"><img src="https://github.com/eladbash/LintScout/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="https://crates.io/crates/lintscout"><img src="https://img.shields.io/crates/v/lintscout.svg" alt="crates.io"></a>
    <a href="https://github.com/eladbash/LintScout/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
    <a href="https://github.com/eladbash/LintScout"><img src="https://img.shields.io/badge/rust-1.74%2B-orange.svg" alt="Rust 1.74+"></a>
  </p>
</p>

---

`// eslint-disable-next-line` ... `# noqa` ... `// NOLINT` ... `@SuppressWarnings` ...

These one-line comments silently bypass your linters, and over time they pile up. **LintScout** scans your codebase, finds every suppression directive, and reports them so you can track, review, and control your tech debt.

```
$ lintscout src/

src/api/handler.ts:42 [eslint:eslint-disable-next-line] ESLint disable next line (suppresses: @typescript-eslint/no-explicit-any)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
src/api/handler.ts:88 [typescript:ts-ignore] TypeScript ignore directive
    // @ts-ignore
src/utils/legacy.py:12 [flake8:noqa] Flake8 noqa directive
    import os  # noqa
src/utils/legacy.py:19 [bandit:nosec] Bandit nosec directive
    x = eval(cmd)  # nosec

Files walked: 214, scanned: 87, skipped: 127
Findings: 4
Duration: 12ms
```

## Why LintScout?

Linters exist for a reason. Every `eslint-disable` or `# noqa` is a decision to bypass a safety net. Sometimes that's fine -- but those decisions should be **visible, tracked, and intentional**, not quietly buried in old pull requests.

LintScout helps you:

- **Audit** -- see every suppressed warning across your entire codebase in seconds
- **Gate CI** -- fail builds when ignore directives exceed a threshold you control
- **Track trends** -- use JSON or SARIF output to feed dashboards and track tech debt over time
- **Onboard** -- give new team members instant visibility into where shortcuts were taken

## Quick Start

### Install

```bash
cargo install lintscout
```

Or build from source:

```bash
git clone https://github.com/eladbash/LintScout.git
cd LintScout
cargo install --path .
```

### Run

```bash
# Scan current directory
lintscout

# Scan a specific path
lintscout src/

# JSON output (great for CI and dashboards)
lintscout --format json

# Fail CI if there are any findings
lintscout --pass-threshold 0

# Only check for Python issues
lintscout --scouts pylint,flake8,mypy,bandit
```

## Supported Linters

LintScout ships with **27 built-in scouts** covering **14 language ecosystems**:

### JavaScript / TypeScript
| Scout | Detects | File types |
|-------|---------|------------|
| **eslint** | `eslint-disable`, `eslint-disable-next-line`, `eslint-disable-line`, `eslint-enable` | `.js` `.jsx` `.ts` `.tsx` `.mjs` `.cjs` `.vue` `.svelte` |
| **oxlint** | `oxlint-disable`, `oxlint-disable-next-line`, `oxlint-disable-line`, `oxlint-enable` | `.js` `.jsx` `.ts` `.tsx` `.mjs` `.cjs` `.vue` `.svelte` |
| **typescript** | `@ts-ignore`, `@ts-nocheck`, `@ts-expect-error` | `.ts` `.tsx` |
| **biome** | `biome-ignore` | `.js` `.jsx` `.ts` `.tsx` `.json` `.jsonc` |
| **prettier** | `prettier-ignore` | `.js` `.jsx` `.ts` `.tsx` `.css` `.scss` `.html` `.md` `.json` `.yaml` `.yml` `.vue` `.svelte` |
| **jshint** | `jshint ignore` | `.js` |

### Python
| Scout | Detects | File types |
|-------|---------|------------|
| **pylint** | `pylint: disable`, `pylint: disable-next` | `.py` |
| **flake8** | `# noqa` | `.py` |
| **ruff** | `# ruff: noqa` | `.py` |
| **mypy** | `# type: ignore` | `.py` `.pyi` |
| **pyright** | `# pyright: ignore` | `.py` `.pyi` |
| **bandit** | `# nosec` | `.py` |

### Go
| Scout | Detects | File types |
|-------|---------|------------|
| **golangci-lint** | `//nolint` | `.go` |
| **gosec** | `//#nosec` | `.go` |
| **staticcheck** | `//lint:ignore`, `//lint:file-ignore` | `.go` |

### Java / Kotlin
| Scout | Detects | File types |
|-------|---------|------------|
| **java** | `@SuppressWarnings`, `CHECKSTYLE: OFF/ON`, `NOPMD`, `@SuppressFBWarnings` | `.java` |
| **detekt** | `@Suppress(...)`, `@file:Suppress(...)` | `.kt` `.kts` |
| **ktlint** | `ktlint-disable`, `@Suppress("ktlint:...")` | `.kt` `.kts` |

### Rust
| Scout | Detects | File types |
|-------|---------|------------|
| **clippy** | `#[allow(clippy::...)]`, `#![allow(clippy::...)]` | `.rs` |

### C / C++
| Scout | Detects | File types |
|-------|---------|------------|
| **clang-tidy** | `NOLINT`, `NOLINTNEXTLINE`, `NOLINTBEGIN`, `NOLINTEND` | `.c` `.cc` `.cpp` `.cxx` `.h` `.hpp` `.hxx` |
| **cppcheck** | `cppcheck-suppress` | `.c` `.cc` `.cpp` `.cxx` `.h` `.hpp` `.hxx` |

### PHP
| Scout | Detects | File types |
|-------|---------|------------|
| **phpstan** | `@phpstan-ignore-next-line`, `@phpstan-ignore-line`, `@phpstan-ignore` | `.php` |

### Ruby / CSS / Shell / Dockerfile / Swift
| Scout | Detects | File types |
|-------|---------|------------|
| **rubocop** | `rubocop:disable`, `rubocop:enable`, `rubocop:todo` | `.rb` `.rake` `.gemspec` |
| **stylelint** | `stylelint-disable`, `stylelint-enable` | `.css` `.scss` `.sass` `.less` `.vue` `.svelte` |
| **shellcheck** | `shellcheck disable=` | `.sh` `.bash` `.zsh` `.ksh` |
| **hadolint** | `hadolint ignore=` | `Dockerfile` |
| **swiftlint** | `swiftlint:disable`, `swiftlint:enable` | `.swift` |

## CLI Reference

```
lintscout [OPTIONS] [PATH]
```

| Option | Default | Description |
|--------|---------|-------------|
| `[PATH]` | `.` | Directory or file to scan |
| `--format <FORMAT>` | `text` | Output format: `text`, `json`, `count`, or `sarif` |
| `--config <PATH>` | auto-detect | Path to config file |
| `--pass-threshold <N>` | none | Exit 0 if findings <= N |
| `--scouts <LIST>` | all | Only run these scouts (comma-separated) |
| `--exclude-scouts <LIST>` | none | Skip these scouts (comma-separated) |
| `--exclude <LIST>` | from config | Exclude these paths (comma-separated) |
| `--no-gitignore` | false | Don't respect `.gitignore` files |
| `--quiet` | false | Suppress output when there are no findings |

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | No findings (or findings <= pass threshold) |
| `1` | Findings exceed threshold |
| `2` | Runtime error (bad config, I/O failure, etc.) |

## Configuration

LintScout automatically looks for `.lintscout.yml` or `lintscout.yml` in the current directory. You can also pass `--config <path>` explicitly.

```yaml
# .lintscout.yml

settings:
  # Directories to skip (default: node_modules, vendor, target, dist)
  exclude:
    - node_modules
    - vendor
    - target
    - dist
    - build

  # Whether to honor .gitignore files (default: true)
  respect_gitignore: true

  # Default output format: text | json | count | sarif
  output: text

  # If set, exit 0 when findings <= this number
  pass_threshold: 10

  # Disable specific built-in scouts
  disable:
    scouts:
      - jshint
      - prettier

# Define your own custom scouts
scouts:
  - name: no-debug-prints
    linter: custom
    language: python
    extensions: [py]
    rules:
      - id: print-statement
        description: "Debug print statement"
        pattern: "^\\s*print\\("

  - name: todo-fixme
    linter: custom
    language: general
    extensions: [js, ts, py, go, rs, java, rb]
    rules:
      - id: todo
        description: "TODO comment"
        pattern: "TODO"
      - id: fixme
        description: "FIXME comment"
        pattern: "FIXME"

  - name: custom-noqa
    linter: custom
    language: python
    extensions: [py]
    rules:
      - id: custom-ignore
        description: "Custom ignore directive"
        pattern: "# custom-ignore"
        # Optional: extract which rules are suppressed
        capture_pattern: "# custom-ignore:\\s*(.+)"
```

### Config Precedence

CLI flags always override config file values:

```bash
# Config says output: text, but this overrides to json
lintscout --format json

# Config says pass_threshold: 10, but this overrides to 0
lintscout --pass-threshold 0
```

## CI/CD Integration

### GitHub Actions

```yaml
name: LintScout

on:
  pull_request:

jobs:
  lint-ignores:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install lintscout

      # Fail if any lint ignores are found
      - run: lintscout --pass-threshold 0 .

      # Or: allow up to 20 existing ignores
      # - run: lintscout --pass-threshold 20 .
```

### GitLab CI

```yaml
lint-ignores:
  stage: test
  image: rust:latest
  script:
    - cargo install lintscout
    - lintscout --pass-threshold 0 .
```

### JSON Output for Dashboards

```bash
# Pipe JSON output to your metrics system
lintscout --format json | jdx -Q '.stats.findings_count' --non-interactive

# Save a snapshot
lintscout --format json > lintscout-report.json
```

## Output Formats

### Text (default)

```
src/handler.ts:42 [eslint:eslint-disable-next-line] ESLint disable next line (suppresses: @typescript-eslint/no-explicit-any)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any

Files walked: 214, scanned: 87, skipped: 127
Findings: 1
Duration: 12ms
```

When a directive specifies which rules it suppresses, LintScout extracts and displays them in the `(suppresses: ...)` annotation.

### JSON

```json
{
  "findings": [
    {
      "path": "src/handler.ts",
      "line_number": 42,
      "line_text": "// eslint-disable-next-line @typescript-eslint/no-explicit-any",
      "scout_name": "eslint",
      "linter": "eslint",
      "rule_id": "eslint-disable-next-line",
      "rule_description": "ESLint disable next line",
      "suppressed_rules": ["@typescript-eslint/no-explicit-any"]
    }
  ],
  "stats": {
    "files_walked": 214,
    "files_scanned": 87,
    "files_skipped": 127,
    "findings_count": 1,
    "errors_count": 0,
    "duration_ms": 12
  }
}
```

The `suppressed_rules` field is only present when the directive specifies which rules it suppresses. Bare directives like `# noqa` or `// @ts-ignore` omit it.

### SARIF

[SARIF v2.1.0](https://sarifweb.azurewebsites.net/) output for integration with GitHub Code Scanning, VS Code SARIF Viewer, and other SARIF-compatible tools:

```bash
lintscout --format sarif > report.sarif
```

```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [{
    "tool": {
      "driver": {
        "name": "lintscout",
        "version": "0.1.0",
        "rules": [{ "id": "eslint/eslint-disable-next-line", "shortDescription": { "text": "ESLint disable next line" } }]
      }
    },
    "results": [{ "ruleId": "eslint/eslint-disable-next-line", "ruleIndex": 0, "message": { "text": "..." }, "locations": [{ "physicalLocation": { "artifactLocation": { "uri": "src/handler.ts" }, "region": { "startLine": 42 } } }] }]
  }]
}
```

### Count

```
1
```

## Contributing

Contributions are welcome! Here's how to get started:

### Development Setup

```bash
git clone https://github.com/eladbash/LintScout.git
cd LintScout
cargo build
cargo test
```

### Adding a New Scout

Adding support for a new linter takes about 5 minutes:

1. Create `src/builtin/my_linter.rs`:

```rust
use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "my-linter".into(),
        linter: "my-linter".into(),
        language: "my-language".into(),
        extensions: vec!["ext1".into(), "ext2".into()],
        rules: vec![
            Rule::new(
                "rule-id",
                "Human-readable description",
                r"regex-pattern-here",
            )?
            // Optional: extract suppressed rule names from the directive
            .with_capture(r"my-linter-ignore\s+(.+)")?,
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scout_compiles() {
        assert!(scout().is_ok());
    }

    #[test]
    fn matches_expected_directives() {
        let s = scout().unwrap();
        assert!(!s.find_matches("// my-linter-ignore").is_empty());
    }
}
```

2. Register it in `src/builtin/mod.rs`:

```rust
mod my_linter;  // add this line

// In the all() function, add:
my_linter::scout()?,
```

3. Run `cargo test` to verify everything works.

### Pull Request Checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes (all tests green)
- [ ] New scouts include tests for both true positives and false negatives
- [ ] Test fixture files added to `tests/fixtures/` if applicable

### Architecture

```
src/
  main.rs           CLI entrypoint
  lib.rs            Public library API
  cli.rs            Argument parsing (clap)
  error.rs          Error types (thiserror)
  rule.rs           Regex-based detection rule
  scout.rs          Scout: groups rules + file matching
  finding.rs        Scan result data structure
  stats.rs          Scan statistics
  scanner.rs        Filesystem walker + matching engine
  config.rs         YAML config loading
  registry.rs       Scout registry (builtins + custom)
  builtin/          27 built-in scout definitions
  output/           Text, JSON, count, and SARIF formatters
```

## Performance

LintScout is built in Rust for speed. It uses the [`ignore`](https://docs.rs/ignore) crate (the same engine behind [ripgrep](https://github.com/BurntSushi/ripgrep)) for filesystem traversal, [`rayon`](https://docs.rs/rayon) for parallel file processing, and respects `.gitignore` by default.

Typical performance on a mid-size codebase (~10k files):

- **Scan time**: <100ms
- **Memory**: <10MB RSS

## License

MIT License -- see [LICENSE](LICENSE) for details.

Copyright (c) 2023 Elad Ben Shmuel

---

<p align="center">
  <sub>Built with Rust. Made to keep codebases honest.</sub>
</p>
