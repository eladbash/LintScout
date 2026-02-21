use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{LintScoutError, Result};
use crate::rule::Rule;
use crate::scout::Scout;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub scouts: Vec<CustomScoutConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "default_excludes")]
    pub exclude: Vec<String>,
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default)]
    pub pass_threshold: Option<u64>,
    #[serde(default)]
    pub disable: DisableConfig,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            exclude: default_excludes(),
            respect_gitignore: true,
            output: default_output(),
            pass_threshold: None,
            disable: DisableConfig::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct DisableConfig {
    #[serde(default)]
    pub scouts: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CustomScoutConfig {
    pub name: String,
    #[serde(default = "default_custom")]
    pub linter: String,
    #[serde(default = "default_custom")]
    pub language: String,
    pub extensions: Vec<String>,
    pub rules: Vec<CustomRuleConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CustomRuleConfig {
    pub id: String,
    pub description: String,
    pub pattern: String,
}

fn default_excludes() -> Vec<String> {
    vec![
        "node_modules".into(),
        "vendor".into(),
        "target".into(),
        "dist".into(),
    ]
}

fn default_true() -> bool {
    true
}

fn default_output() -> String {
    "text".into()
}

fn default_custom() -> String {
    "custom".into()
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        let content = std::fs::read_to_string(path).map_err(|e| LintScoutError::ConfigLoad {
            path: path.to_path_buf(),
            source: Box::new(e),
        })?;
        let config: Config =
            serde_yaml::from_str(&content).map_err(|e| LintScoutError::ConfigLoad {
                path: path.to_path_buf(),
                source: Box::new(e),
            })?;
        config.validate()?;
        Ok(config)
    }

    pub fn find_and_load() -> Option<Result<Config>> {
        let candidates = [".lintscout.yml", "lintscout.yml"];
        for name in &candidates {
            let path = PathBuf::from(name);
            if path.exists() {
                return Some(Config::load(&path));
            }
        }
        None
    }

    fn validate(&self) -> Result<()> {
        for scout_cfg in &self.scouts {
            if scout_cfg.extensions.is_empty() {
                return Err(LintScoutError::Config(format!(
                    "custom scout '{}' must have at least one extension",
                    scout_cfg.name
                )));
            }
            if scout_cfg.rules.is_empty() {
                return Err(LintScoutError::Config(format!(
                    "custom scout '{}' must have at least one rule",
                    scout_cfg.name
                )));
            }
            for rule_cfg in &scout_cfg.rules {
                regex::Regex::new(&rule_cfg.pattern).map_err(|e| {
                    LintScoutError::InvalidPattern {
                        pattern: rule_cfg.pattern.clone(),
                        source: e,
                    }
                })?;
            }
        }
        Ok(())
    }

    pub fn build_custom_scouts(&self) -> Result<Vec<Scout>> {
        let mut scouts = Vec::new();
        for cfg in &self.scouts {
            let mut rules = Vec::new();
            for r in &cfg.rules {
                rules.push(Rule::new(&r.id, &r.description, &r.pattern)?);
            }
            scouts.push(Scout {
                name: cfg.name.clone(),
                linter: cfg.linter.clone(),
                language: cfg.language.clone(),
                extensions: cfg.extensions.clone(),
                rules,
            });
        }
        Ok(scouts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_config() {
        let yaml = r#"
settings:
  exclude:
    - node_modules
  output: json
  pass_threshold: 5
scouts:
  - name: custom-test
    extensions: [ts]
    rules:
      - id: test-rule
        description: "A test rule"
        pattern: "test-pattern"
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.settings.output, "json");
        assert_eq!(config.settings.pass_threshold, Some(5));
        assert_eq!(config.scouts.len(), 1);
        assert_eq!(config.scouts[0].name, "custom-test");
    }

    #[test]
    fn custom_scouts_build() {
        let yaml = r#"
scouts:
  - name: my-scout
    extensions: [py]
    rules:
      - id: r1
        description: "Rule 1"
        pattern: "foo.*bar"
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        let scouts = config.build_custom_scouts().unwrap();
        assert_eq!(scouts.len(), 1);
        assert_eq!(scouts[0].name, "my-scout");
    }

    #[test]
    fn invalid_regex_in_config() {
        let yaml = r#"
scouts:
  - name: bad
    extensions: [py]
    rules:
      - id: r1
        description: "Bad rule"
        pattern: "[invalid"
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn missing_extensions_error() {
        let yaml = r#"
scouts:
  - name: bad
    extensions: []
    rules:
      - id: r1
        description: "Rule"
        pattern: "foo"
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        let result = config.validate();
        assert!(result.is_err());
    }
}
