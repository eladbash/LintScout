use regex::Regex;

use crate::error::{LintScoutError, Result};

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub description: String,
    pub pattern: Regex,
    pub pattern_str: String,
}

impl Rule {
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        pattern_str: impl Into<String>,
    ) -> Result<Rule> {
        let pattern_str = pattern_str.into();
        let pattern = Regex::new(&pattern_str).map_err(|e| LintScoutError::InvalidPattern {
            pattern: pattern_str.clone(),
            source: e,
        })?;
        Ok(Rule {
            id: id.into(),
            description: description.into(),
            pattern,
            pattern_str,
        })
    }

    pub fn is_match(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_pattern() {
        let rule = Rule::new("test", "A test rule", r"eslint-disable").unwrap();
        assert!(rule.is_match("// eslint-disable-next-line"));
        assert!(!rule.is_match("normal code"));
    }

    #[test]
    fn invalid_pattern() {
        let result = Rule::new("bad", "Bad pattern", r"[invalid");
        assert!(result.is_err());
    }

    #[test]
    fn pattern_str_preserved() {
        let rule = Rule::new("t", "t", r"foo\d+").unwrap();
        assert_eq!(rule.pattern_str, r"foo\d+");
    }
}
