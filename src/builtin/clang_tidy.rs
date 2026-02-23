use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "clang-tidy".into(),
        linter: "clang-tidy".into(),
        language: "cpp".into(),
        extensions: vec![
            "c".into(),
            "cc".into(),
            "cpp".into(),
            "cxx".into(),
            "h".into(),
            "hpp".into(),
            "hxx".into(),
        ],
        rules: vec![
            Rule::new("nolint", "clang-tidy NOLINT directive", r"NOLINT(\(|$|\s)")?
                .with_capture(r"NOLINT\(([^)]+)\)")?,
            Rule::new(
                "nolintnextline",
                "clang-tidy NOLINTNEXTLINE directive",
                r"NOLINTNEXTLINE",
            )?
            .with_capture(r"NOLINTNEXTLINE\(([^)]+)\)")?,
            Rule::new(
                "nolintbegin",
                "clang-tidy NOLINTBEGIN directive",
                r"NOLINTBEGIN",
            )?
            .with_capture(r"NOLINTBEGIN\(([^)]+)\)")?,
            Rule::new("nolintend", "clang-tidy NOLINTEND directive", r"NOLINTEND")?,
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
    fn matches_nolint_directives() {
        let s = scout().unwrap();
        assert!(!s.find_matches("int x = 1; // NOLINT").is_empty());
        assert!(!s
            .find_matches("int x = 1; // NOLINT(cert-err58-cpp)")
            .is_empty());
        assert!(!s.find_matches("// NOLINTNEXTLINE").is_empty());
        assert!(!s.find_matches("// NOLINTBEGIN").is_empty());
        assert!(!s.find_matches("// NOLINTEND").is_empty());
    }
}
