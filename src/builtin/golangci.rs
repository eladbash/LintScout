use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "golangci-lint".into(),
        linter: "golangci-lint".into(),
        language: "go".into(),
        extensions: vec!["go".into()],
        rules: vec![
            Rule::new("nolint", "golangci-lint nolint directive", r"//\s*nolint")?
                .with_capture(r"//\s*nolint:([^\s]+)")?,
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
    fn matches_nolint() {
        let s = scout().unwrap();
        assert!(!s.find_matches("x := 1 //nolint:errcheck").is_empty());
        assert!(!s.find_matches("x := 1 // nolint").is_empty());
    }
}
