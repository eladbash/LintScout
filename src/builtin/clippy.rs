use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "clippy".into(),
        linter: "clippy".into(),
        language: "rust".into(),
        extensions: vec!["rs".into()],
        rules: vec![
            Rule::new(
                "allow-clippy",
                "Clippy allow attribute",
                r"#\[allow\(clippy::",
            )?
            .with_capture(r"#\[allow\(clippy::([^)]+)\)")?,
            Rule::new(
                "allow-clippy-file",
                "Clippy file-level allow attribute",
                r"#!\[allow\(clippy::",
            )?
            .with_capture(r"#!\[allow\(clippy::([^)]+)\)")?,
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
    fn matches_clippy_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("#[allow(clippy::needless_return)]")
            .is_empty());
        assert!(!s.find_matches("#![allow(clippy::all)]").is_empty());
        assert!(!s
            .find_matches("#[allow(clippy::too_many_arguments)]")
            .is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("#[allow(dead_code)]").is_empty());
        assert!(s.find_matches("use clippy;").is_empty());
    }
}
