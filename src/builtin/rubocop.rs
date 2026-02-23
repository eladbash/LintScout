use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "rubocop".into(),
        linter: "rubocop".into(),
        language: "ruby".into(),
        extensions: vec!["rb".into(), "rake".into(), "gemspec".into()],
        rules: vec![Rule::new(
            "rubocop-directive",
            "RuboCop disable/enable/todo directive",
            r"rubocop:(disable|enable|todo)",
        )?
        .with_capture(r"rubocop:(?:disable|enable|todo)\s+(.+)")?],
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
    fn matches_rubocop_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("# rubocop:disable Style/NumericLiterals")
            .is_empty());
        assert!(!s.find_matches("# rubocop:enable all").is_empty());
        assert!(!s
            .find_matches("# rubocop:todo Metrics/MethodLength")
            .is_empty());
    }
}
