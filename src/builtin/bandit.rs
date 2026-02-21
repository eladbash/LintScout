use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "bandit".into(),
        linter: "bandit".into(),
        language: "python".into(),
        extensions: vec!["py".into()],
        rules: vec![Rule::new("nosec", "Bandit nosec directive", r"#\s*nosec")?],
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
        assert!(!s.find_matches("x = eval('1')  # nosec").is_empty());
        assert!(!s.find_matches("x = eval('1')  # nosec B307").is_empty());
    }
}
