use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "java".into(),
        linter: "java".into(),
        language: "java".into(),
        extensions: vec!["java".into()],
        rules: vec![
            Rule::new(
                "suppress-warnings",
                "Java SuppressWarnings annotation",
                r"@SuppressWarnings",
            )?
            .with_capture(r#"@SuppressWarnings\(\{?"([^"]*)"#)?,
            Rule::new(
                "checkstyle-off",
                "Checkstyle toggle directive",
                r"CHECKSTYLE:\s*(OFF|ON)",
            )?,
            Rule::new("nopmd", "PMD suppression directive", r"NOPMD")?,
            Rule::new(
                "suppress-fb-warnings",
                "FindBugs/SpotBugs suppression",
                r"@SuppressFBWarnings",
            )?
            .with_capture(r#"@SuppressFBWarnings\("([^"]*)""#)?,
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
    fn matches_java_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("@SuppressWarnings(\"unchecked\")")
            .is_empty());
        assert!(!s.find_matches("// CHECKSTYLE: OFF").is_empty());
        assert!(!s.find_matches("// CHECKSTYLE: ON").is_empty());
        assert!(!s.find_matches("} // NOPMD").is_empty());
        assert!(!s.find_matches("@SuppressFBWarnings(\"NP\")").is_empty());
    }
}
