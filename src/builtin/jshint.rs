use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "jshint".into(),
        linter: "jshint".into(),
        language: "javascript".into(),
        extensions: vec!["js".into()],
        rules: vec![Rule::new(
            "jshint-ignore",
            "JSHint ignore directive",
            r"jshint\s+ignore",
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
    fn matches_jshint_directive() {
        let s = scout().unwrap();
        assert!(!s.find_matches("/* jshint ignore:start */").is_empty());
    }
}
