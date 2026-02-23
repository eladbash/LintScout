use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "shellcheck".into(),
        linter: "shellcheck".into(),
        language: "shell".into(),
        extensions: vec!["sh".into(), "bash".into(), "zsh".into(), "ksh".into()],
        rules: vec![Rule::new(
            "shellcheck-disable",
            "ShellCheck disable directive",
            r"shellcheck\s+disable=",
        )?
        .with_capture(r"shellcheck\s+disable=(.+)")?],
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
    fn matches_shellcheck_disable() {
        let s = scout().unwrap();
        assert!(!s.find_matches("# shellcheck disable=SC2034").is_empty());
    }
}
