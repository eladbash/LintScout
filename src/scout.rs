use std::path::Path;

use crate::rule::Rule;

#[derive(Debug, Clone)]
pub struct Scout {
    pub name: String,
    pub linter: String,
    pub language: String,
    pub extensions: Vec<String>,
    pub rules: Vec<Rule>,
}

impl Scout {
    pub fn applies_to_file(&self, path: &Path) -> bool {
        if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
            if self.extensions.iter().any(|ext| ext == fname) {
                return true;
            }
        }
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return self.extensions.iter().any(|e| e == ext);
        }
        false
    }

    pub fn find_matches(&self, line: &str) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.is_match(line)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_scout() -> Scout {
        Scout {
            name: "test".into(),
            linter: "test-linter".into(),
            language: "test-lang".into(),
            extensions: vec!["js".into(), "ts".into(), "Dockerfile".into()],
            rules: vec![
                Rule::new("r1", "rule one", r"eslint-disable").unwrap(),
                Rule::new("r2", "rule two", r"@ts-ignore").unwrap(),
            ],
        }
    }

    #[test]
    fn applies_to_matching_extension() {
        let s = test_scout();
        assert!(s.applies_to_file(Path::new("src/app.js")));
        assert!(s.applies_to_file(Path::new("index.ts")));
    }

    #[test]
    fn does_not_apply_to_wrong_extension() {
        let s = test_scout();
        assert!(!s.applies_to_file(Path::new("styles.css")));
    }

    #[test]
    fn applies_to_filename_match() {
        let s = test_scout();
        assert!(s.applies_to_file(Path::new("path/to/Dockerfile")));
    }

    #[test]
    fn find_matches_returns_matching_rules() {
        let s = test_scout();
        let m = s.find_matches("// eslint-disable-next-line");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].id, "r1");
    }

    #[test]
    fn find_matches_returns_empty_for_no_match() {
        let s = test_scout();
        let m = s.find_matches("normal code");
        assert!(m.is_empty());
    }
}
