use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "eslint".into(),
        linter: "eslint".into(),
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
                "eslint-disable",
                "ESLint disable block directive",
                r"eslint-disable(?:\s|$)",
            )?,
            Rule::new(
                "eslint-disable-next-line",
                "ESLint disable next line",
                r"eslint-disable-next-line",
            )?,
            Rule::new(
                "eslint-disable-line",
                "ESLint disable current line",
                r"eslint-disable-line",
            )?,
            Rule::new(
                "eslint-enable",
                "ESLint re-enable directive",
                r"eslint-enable",
            )?,
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
    fn matches_eslint_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("// eslint-disable-next-line no-unused-vars")
            .is_empty());
        assert!(!s.find_matches("/* eslint-disable */").is_empty());
        assert!(!s.find_matches("/* eslint-enable */").is_empty());
        assert!(!s.find_matches("// eslint-disable-line no-alert").is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s
            .find_matches("const eslint = require('eslint');")
            .is_empty());
        assert!(s.find_matches("normal code").is_empty());
    }
}
