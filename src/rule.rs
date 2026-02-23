use regex::Regex;

use crate::error::{LintScoutError, Result};

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub description: String,
    pub pattern: Regex,
    pub pattern_str: String,
    pub capture_pattern: Option<Regex>,
    pub capture_pattern_str: Option<String>,
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
            capture_pattern: None,
            capture_pattern_str: None,
        })
    }

    pub fn with_capture(mut self, pattern: &str) -> Result<Self> {
        let regex = Regex::new(pattern).map_err(|e| LintScoutError::InvalidPattern {
            pattern: pattern.to_string(),
            source: e,
        })?;
        self.capture_pattern = Some(regex);
        self.capture_pattern_str = Some(pattern.to_string());
        Ok(self)
    }

    pub fn is_match(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }

    pub fn capture_suppressed_rules(&self, line: &str) -> Option<Vec<String>> {
        let cap_re = self.capture_pattern.as_ref()?;
        let caps = cap_re.captures(line)?;
        let group = caps.get(1)?;
        let rules: Vec<String> = group
            .as_str()
            .split([',', ' '])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        if rules.is_empty() {
            None
        } else {
            Some(rules)
        }
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

    #[test]
    fn capture_suppressed_rules_extracts() {
        let rule = Rule::new("test", "test", r"eslint-disable-next-line")
            .unwrap()
            .with_capture(r"eslint-disable-next-line\s+(.+)")
            .unwrap();
        let result =
            rule.capture_suppressed_rules("// eslint-disable-next-line no-alert, no-console");
        assert_eq!(result, Some(vec!["no-alert".into(), "no-console".into()]));
    }

    #[test]
    fn capture_returns_none_without_pattern() {
        let rule = Rule::new("test", "test", r"eslint-disable").unwrap();
        assert!(rule.capture_suppressed_rules("// eslint-disable").is_none());
    }

    #[test]
    fn capture_returns_none_when_no_match() {
        let rule = Rule::new("test", "test", r"eslint-disable")
            .unwrap()
            .with_capture(r"eslint-disable-next-line\s+(.+)")
            .unwrap();
        assert!(rule.capture_suppressed_rules("// eslint-disable").is_none());
    }

    #[test]
    fn invalid_capture_pattern() {
        let result = Rule::new("test", "test", r"foo")
            .unwrap()
            .with_capture(r"[invalid");
        assert!(result.is_err());
    }
}
