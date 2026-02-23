use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "staticcheck".into(),
        linter: "staticcheck".into(),
        language: "go".into(),
        extensions: vec!["go".into()],
        rules: vec![Rule::new(
            "lint-ignore",
            "staticcheck file-ignore or ignore directive",
            r"//lint:(file-)?ignore",
        )?
        .with_capture(r"//lint:(?:file-)?ignore\s+(\S+)")?],
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
    fn matches_lint_ignore() {
        let s = scout().unwrap();
        assert!(!s.find_matches("//lint:ignore SA1000 reason").is_empty());
        assert!(!s
            .find_matches("//lint:file-ignore SA1000 reason")
            .is_empty());
    }
}
