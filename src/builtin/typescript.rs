use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "typescript".into(),
        linter: "typescript".into(),
        language: "typescript".into(),
        extensions: vec!["ts".into(), "tsx".into()],
        rules: vec![
            Rule::new("ts-ignore", "TypeScript ignore directive", r"@ts-ignore")?,
            Rule::new("ts-nocheck", "TypeScript nocheck directive", r"@ts-nocheck")?,
            Rule::new(
                "ts-expect-error",
                "TypeScript expect-error directive",
                r"@ts-expect-error",
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
    fn matches_ts_directives() {
        let s = scout().unwrap();
        assert!(!s.find_matches("// @ts-ignore").is_empty());
        assert!(!s.find_matches("// @ts-nocheck").is_empty());
        assert!(!s.find_matches("// @ts-expect-error").is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("const tsconfig = {};").is_empty());
    }
}
