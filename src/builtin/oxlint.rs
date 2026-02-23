use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "oxlint".into(),
        linter: "oxlint".into(),
        language: "javascript".into(),
        extensions: vec![
            "js".into(),
            "jsx".into(),
            "ts".into(),
            "tsx".into(),
            "mjs".into(),
            "cjs".into(),
            "vue".into(),
            "svelte".into(),
        ],
        rules: vec![
            Rule::new(
                "oxlint-disable",
                "oxlint disable block directive",
                r"oxlint-disable(?:\s|$)",
            )?
            .with_capture(r"oxlint-disable\s+([^*]+)")?,
            Rule::new(
                "oxlint-disable-next-line",
                "oxlint disable next line",
                r"oxlint-disable-next-line",
            )?
            .with_capture(r"oxlint-disable-next-line\s+([^*]+)")?,
            Rule::new(
                "oxlint-disable-line",
                "oxlint disable current line",
                r"oxlint-disable-line",
            )?
            .with_capture(r"oxlint-disable-line\s+([^*]+)")?,
            Rule::new(
                "oxlint-enable",
                "oxlint re-enable directive",
                r"oxlint-enable",
            )?
            .with_capture(r"oxlint-enable\s+([^*]+)")?,
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
    fn matches_oxlint_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("// oxlint-disable-next-line no-unused-vars")
            .is_empty());
        assert!(!s.find_matches("/* oxlint-disable */").is_empty());
        assert!(!s.find_matches("/* oxlint-enable */").is_empty());
        assert!(!s.find_matches("// oxlint-disable-line no-alert").is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s
            .find_matches("const oxlint = require('oxlint');")
            .is_empty());
        assert!(s.find_matches("normal code").is_empty());
    }
}
