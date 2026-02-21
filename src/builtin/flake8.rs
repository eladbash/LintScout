use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "flake8".into(),
        linter: "flake8".into(),
        language: "python".into(),
        extensions: vec!["py".into()],
        rules: vec![Rule::new("noqa", "Flake8 noqa directive", r"#\s*noqa")?],
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
    fn matches_noqa() {
        let s = scout().unwrap();
        assert!(!s.find_matches("import os  # noqa").is_empty());
        assert!(!s.find_matches("import os  # noqa: F401").is_empty());
    }
}
