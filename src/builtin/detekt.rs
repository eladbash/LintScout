use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "detekt".into(),
        linter: "detekt".into(),
        language: "kotlin".into(),
        extensions: vec!["kt".into(), "kts".into()],
        rules: vec![
            Rule::new(
                "suppress",
                "Kotlin/Detekt Suppress annotation",
                r"@Suppress\(",
            )?
            .with_capture(r#"@Suppress\("([^"]*)"#)?,
            Rule::new(
                "file-suppress",
                "Kotlin/Detekt file-level Suppress",
                r"@file:Suppress",
            )?
            .with_capture(r#"@file:Suppress\("([^"]*)"#)?,
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
    fn matches_suppress() {
        let s = scout().unwrap();
        assert!(!s.find_matches("@Suppress(\"MaxLineLength\")").is_empty());
        assert!(!s
            .find_matches("@file:Suppress(\"TooManyFunctions\")")
            .is_empty());
    }
}
