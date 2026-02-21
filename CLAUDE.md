# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

LintScout is a Rust CLI tool that detects linter ignore directives (e.g., `eslint-disable`, `@ts-nocheck`, `# noqa`) in source code. It supports 22 built-in scouts covering 12+ languages and allows custom rules via YAML configuration. Designed for local use and CI/CD integration.

## Build & Run Commands

```bash
cargo build                 # Debug build
cargo build --release       # Release build
cargo run                   # Run (scans ./ by default)
cargo test                  # Run all tests (71 unit + integration)
cargo clippy --all-targets -- -D warnings  # Lint check (deny warnings)
cargo fmt                   # Format code
cargo publish --dry-run     # Verify crates.io readiness
```

## Architecture

### Core types (flat modules in `src/`)
- **`rule.rs`** — `Rule` struct: regex-based detection rule with `id`, `description`, `pattern`. `Rule::new()` returns `Result` instead of panicking.
- **`scout.rs`** — `Scout` struct: groups rules with name, linter, language, extensions. `applies_to_file()` checks extension or filename. `find_matches()` returns matching rules.
- **`finding.rs`** — `Finding` struct: owned data (no lifetimes) with path, line_number, line_text, scout_name, linter, rule_id, rule_description. Derives `Serialize`.
- **`stats.rs`** — `ScanStats` struct: files_walked, files_scanned, files_skipped, findings_count, errors_count, duration_ms.
- **`error.rs`** — `LintScoutError` enum with `thiserror`: FileRead, InvalidPattern, Config, ConfigLoad, Io. Type alias `Result<T>`.

### `src/builtin/` — 22 built-in scout implementations
Each file exports `pub fn scout() -> Result<Scout>`. Registered in `builtin/mod.rs` via `pub fn all() -> Result<Vec<Scout>>`.

Scouts: eslint, typescript, biome, prettier, jshint, pylint, flake8, mypy, pyright, bandit, golangci, gosec, staticcheck, java, detekt, rubocop, clang_tidy, cppcheck, stylelint, shellcheck, hadolint, swiftlint.

### `src/scanner.rs` — Filesystem scanning engine
`Scanner` struct with builder pattern. Uses `ignore::WalkBuilder` with exclude overrides. Collects errors instead of panicking. Returns `ScanResult { findings, stats, errors }`.

### `src/config.rs` — YAML configuration
Loads `.lintscout.yml` or `lintscout.yml`. Supports custom scouts, exclude paths, output format, pass threshold, and scout disabling.

### `src/registry.rs` — Scout registry
Builder: `ScoutRegistry::new().with_builtins().with_config(&config).filter(names).exclude(names).into_scouts()`.

### `src/cli.rs` — CLI argument parsing (clap derive)
### `src/output/` — Output formatters (text, json, count)

### Data flow
```
main.rs → parse CLI → load config → build registry → create Scanner → scanner.run()
  → walks filesystem → matches files to scouts → applies rules per line
  → returns ScanResult → format output → exit code
```

## Adding a New Scout

1. Create `src/builtin/<linter>.rs` with `pub fn scout() -> Result<Scout>`.
2. Define rules with `Rule::new(id, description, regex_pattern)?`.
3. Register in `src/builtin/mod.rs` (add `mod` declaration and call in `all()`).
4. Add tests in the same file.

## Dependencies

- **`clap`**: CLI argument parsing
- **`ignore`**: Filesystem walker with .gitignore support
- **`regex`**: Pattern matching for scout rules
- **`serde` + `serde_yaml` + `serde_json`**: Config parsing and JSON output
- **`thiserror`**: Error type derivation

## Testing

- 61 unit tests (in-module `#[cfg(test)]`) covering rules, scouts, config, and all builtins
- 10 integration tests (`tests/cli_tests.rs`) covering CLI options, output formats, exit codes
- Test fixtures in `tests/fixtures/` with sample files for each supported language
