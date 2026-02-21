use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "cppcheck".into(),
        linter: "cppcheck".into(),
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
        rules: vec![Rule::new(
            "cppcheck-suppress",
            "cppcheck suppress directive",
            r"cppcheck-suppress",
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
    fn matches_cppcheck_suppress() {
        let s = scout().unwrap();
        assert!(!s.find_matches("// cppcheck-suppress uninitvar").is_empty());
    }
}
