use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "ktlint".into(),
        linter: "ktlint".into(),
        language: "kotlin".into(),
        extensions: vec!["kt".into(), "kts".into()],
        rules: vec![
            Rule::new(
                "ktlint-disable",
                "ktlint disable directive",
                r"ktlint-disable",
            )?
            .with_capture(r"ktlint-disable\s+(.+)")?,
            Rule::new(
                "suppress-ktlint",
                "Kotlin Suppress annotation for ktlint",
                r#"@Suppress\("ktlint:"#,
            )?
            .with_capture(r#"@Suppress\("ktlint:([^"]+)"#)?,
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
    fn matches_ktlint_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("// ktlint-disable no-wildcard-imports")
            .is_empty());
        assert!(!s
            .find_matches(r#"@Suppress("ktlint:standard:no-wildcard-imports")"#)
            .is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("import ktlint").is_empty());
    }
}
