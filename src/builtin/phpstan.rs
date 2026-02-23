use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "phpstan".into(),
        linter: "phpstan".into(),
        language: "php".into(),
        extensions: vec!["php".into()],
        rules: vec![
            Rule::new(
                "phpstan-ignore-next-line",
                "PHPStan ignore next line directive",
                r"@phpstan-ignore-next-line",
            )?,
            Rule::new(
                "phpstan-ignore-line",
                "PHPStan ignore current line directive",
                r"@phpstan-ignore-line",
            )?,
            Rule::new(
                "phpstan-ignore",
                "PHPStan ignore directive",
                r"@phpstan-ignore\s",
            )?
            .with_capture(r"@phpstan-ignore\s+(.+)")?,
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
    fn matches_phpstan_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("/** @phpstan-ignore-next-line */")
            .is_empty());
        assert!(!s.find_matches("// @phpstan-ignore-line").is_empty());
        assert!(!s
            .find_matches("// @phpstan-ignore argument.type")
            .is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("use PHPStan;").is_empty());
    }
}
