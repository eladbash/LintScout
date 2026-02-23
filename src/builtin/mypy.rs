use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "mypy".into(),
        linter: "mypy".into(),
        language: "python".into(),
        extensions: vec!["py".into(), "pyi".into()],
        rules: vec![Rule::new(
            "type-ignore",
            "Mypy type ignore directive",
            r"#\s*type:\s*ignore",
        )?
        .with_capture(r"#\s*type:\s*ignore\[([^\]]+)\]")?],
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
    fn matches_type_ignore() {
        let s = scout().unwrap();
        assert!(!s.find_matches("x = 1  # type: ignore").is_empty());
        assert!(!s
            .find_matches("x = 1  # type: ignore[attr-defined]")
            .is_empty());
    }
}
