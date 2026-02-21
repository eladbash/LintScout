use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "swiftlint".into(),
        linter: "swiftlint".into(),
        language: "swift".into(),
        extensions: vec!["swift".into()],
        rules: vec![Rule::new(
            "swiftlint-directive",
            "SwiftLint disable/enable directive",
            r"swiftlint:(disable|enable)",
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
    fn matches_swiftlint_directives() {
        let s = scout().unwrap();
        assert!(!s.find_matches("// swiftlint:disable force_cast").is_empty());
        assert!(!s.find_matches("// swiftlint:enable force_cast").is_empty());
    }
}
