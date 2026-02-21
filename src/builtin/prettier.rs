use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "prettier".into(),
        linter: "prettier".into(),
        language: "javascript".into(),
        extensions: vec![
            "js".into(),
            "jsx".into(),
            "ts".into(),
            "tsx".into(),
            "css".into(),
            "scss".into(),
            "html".into(),
            "md".into(),
            "json".into(),
            "yaml".into(),
            "yml".into(),
            "vue".into(),
            "svelte".into(),
        ],
        rules: vec![Rule::new(
            "prettier-ignore",
            "Prettier ignore directive",
            r"prettier-ignore",
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
    fn matches_prettier_directive() {
        let s = scout().unwrap();
        assert!(!s.find_matches("// prettier-ignore").is_empty());
        assert!(!s.find_matches("/* prettier-ignore */").is_empty());
    }
}
