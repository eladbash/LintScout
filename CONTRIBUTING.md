# Contributing to LintScout

Thank you for your interest in contributing to LintScout! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/LintScout.git`
3. Create a branch: `git checkout -b my-feature`
4. Make your changes
5. Run checks: `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`
6. Commit and push
7. Open a pull request

## Development Setup

You need Rust 1.74+ installed. Then:

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo clippy       # Check for lint issues
cargo fmt          # Format code
```

## Adding a New Scout

This is the most common type of contribution. See the [README](README.md#adding-a-new-scout) for step-by-step instructions.

## Reporting Bugs

Open an issue on GitHub with:

- What you expected to happen
- What actually happened
- Steps to reproduce
- Your OS and Rust version (`rustc --version`)

## Suggesting Features

Open an issue with the "feature request" label. Describe the use case and why it would be valuable.

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy --all-targets -- -D warnings` and fix all warnings
- Write tests for new functionality
- Keep commits focused and atomic

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
