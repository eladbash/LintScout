use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "ruff".into(),
        linter: "ruff".into(),
        language: "python".into(),
        extensions: vec!["py".into()],
        rules: vec![
            Rule::new("ruff-noqa", "Ruff noqa directive", r"#\s*ruff:\s*noqa")?
                .with_capture(r"#\s*ruff:\s*noqa:\s*(.+)")?,
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
    fn matches_ruff_noqa() {
        let s = scout().unwrap();
        assert!(!s.find_matches("# ruff: noqa").is_empty());
        assert!(!s.find_matches("# ruff: noqa: F401").is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("import ruff").is_empty());
        assert!(s.find_matches("# noqa: F401").is_empty());
    }
}
