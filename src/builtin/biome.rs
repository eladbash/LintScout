use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "biome".into(),
        linter: "biome".into(),
        language: "javascript".into(),
        extensions: vec![
            "js".into(),
            "jsx".into(),
            "ts".into(),
            "tsx".into(),
            "json".into(),
            "jsonc".into(),
        ],
        rules: vec![Rule::new(
            "biome-ignore",
            "Biome ignore directive",
            r"biome-ignore",
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
    fn matches_biome_directive() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("// biome-ignore lint/style: reason")
            .is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("const biome = 'grassland';").is_empty());
    }
}
