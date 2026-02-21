use crate::error::Result;
use crate::rule::Rule;
use crate::scout::Scout;

pub fn scout() -> Result<Scout> {
    Ok(Scout {
        name: "pylint".into(),
        linter: "pylint".into(),
        language: "python".into(),
        extensions: vec!["py".into()],
        rules: vec![
            Rule::new(
                "pylint-disable",
                "Pylint disable directive",
                r"pylint:\s*disable",
            )?,
            Rule::new(
                "pylint-disable-next",
                "Pylint disable-next directive",
                r"pylint:\s*disable-next",
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
    fn matches_pylint_directives() {
        let s = scout().unwrap();
        assert!(!s
            .find_matches("x = 1  # pylint: disable=invalid-name")
            .is_empty());
        assert!(!s
            .find_matches("# pylint: disable-next=unused-variable")
            .is_empty());
    }

    #[test]
    fn no_false_positives() {
        let s = scout().unwrap();
        assert!(s.find_matches("import pylint").is_empty());
    }
}
