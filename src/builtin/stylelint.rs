use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "stylelint".into(),
        linter: "stylelint".into(),
        language: "css".into(),
        extensions: vec![
            "css".into(),
            "scss".into(),
            "sass".into(),
            "less".into(),
            "vue".into(),
            "svelte".into(),
        ],
        rules: vec![
            Rule::new(
                "stylelint-disable",
                "Stylelint disable directive",
                r"stylelint-disable",
            )?,
            Rule::new(
                "stylelint-enable",
                "Stylelint re-enable directive",
                r"stylelint-enable",
            )?,
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
    fn matches_stylelint_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("/* stylelint-disable color-named */")
            .is_empty());
        assert!(!s.find_matches("/* stylelint-enable */").is_empty());
    }
}
