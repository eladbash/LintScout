use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "pyright".into(),
        linter: "pyright".into(),
        language: "python".into(),
        extensions: vec!["py".into(), "pyi".into()],
        rules: vec![Rule::new(
            "pyright-ignore",
            "Pyright ignore directive",
            r"#\s*pyright:\s*ignore",
        )?],
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
    fn matches_pyright_ignore() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("x = 1  # pyright: ignore[reportGeneralTypeIssues]")
            .is_empty());
    }
}
