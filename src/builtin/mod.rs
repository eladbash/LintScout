mod bandit;
mod biome;
mod clang_tidy;
mod clippy;
mod cppcheck;
mod detekt;
mod eslint;
mod flake8;
mod golangci;
mod gosec;
mod hadolint;
mod java;
mod jshint;
mod ktlint;
mod mypy;
mod oxlint;
mod phpstan;
mod prettier;
mod pylint;
mod pyright;
mod rubocop;
mod ruff;
mod shellcheck;
mod staticcheck;
mod stylelint;
mod swiftlint;
mod typescript;

use crate::error::Result;
use crate::scout::Scout;

pub fn all() -> Result<Vec<Scout>> {
    Ok(vec![
        eslint::scout()?,
        typescript::scout()?,
        biome::scout()?,
        prettier::scout()?,
        jshint::scout()?,
        pylint::scout()?,
        flake8::scout()?,
        mypy::scout()?,
        pyright::scout()?,
        bandit::scout()?,
        golangci::scout()?,
        gosec::scout()?,
        staticcheck::scout()?,
        java::scout()?,
        detekt::scout()?,
        rubocop::scout()?,
        clang_tidy::scout()?,
        cppcheck::scout()?,
        stylelint::scout()?,
        shellcheck::scout()?,
        hadolint::scout()?,
        swiftlint::scout()?,
        ruff::scout()?,
        oxlint::scout()?,
        clippy::scout()?,
        phpstan::scout()?,
        ktlint::scout()?,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtin_patterns_compile() {
        let scouts = all().expect("all builtin scouts should compile");
        assert_eq!(scouts.len(), 27);
        for scout in &scouts {
            assert!(!scout.name.is_empty());
            assert!(!scout.rules.is_empty());
            assert!(!scout.extensions.is_empty());
        }
    }
}
