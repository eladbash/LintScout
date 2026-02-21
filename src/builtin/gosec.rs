use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "gosec".into(),
        linter: "gosec".into(),
        language: "go".into(),
        extensions: vec!["go".into()],
        rules: vec![Rule::new("nosec", "gosec nosec directive", r"//\s*#nosec")?],
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
    fn matches_nosec() {
        let s = scout().unwrap();
        assert!(!s.find_matches("_ = x //#nosec G101").is_empty());
        assert!(!s.find_matches("// #nosec").is_empty());
    }
}
