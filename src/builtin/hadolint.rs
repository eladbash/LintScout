use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "hadolint".into(),
        linter: "hadolint".into(),
        language: "dockerfile".into(),
        extensions: vec!["Dockerfile".into()],
        rules: vec![Rule::new(
            "hadolint-ignore",
            "Hadolint ignore directive",
            r"hadolint\s+ignore=",
        )?
        .with_capture(r"hadolint\s+ignore=(.+)")?],
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
    fn matches_hadolint_ignore() {
        let s = scout().unwrap();
        assert!(!s.find_matches("# hadolint ignore=DL3008").is_empty());
    }
}
