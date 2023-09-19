use regex::Regex;

use super::{fileExtension::FileExtension, language::Language};
#[derive(Debug)]
pub struct Scout {
    language: Language,
    name: String,
    exclude_paths: Vec<Regex>,
    extensions: Vec<FileExtension>,
    rules: Vec<ScoutRule>,
}

impl Scout {
    pub fn new(
        language: Language,
        name: String,
        exclude_paths: Vec<Regex>,
        extensions: Vec<FileExtension>,
        rules: Vec<ScoutRule>,
    ) -> Scout {
        Scout {
            language,
            name,
            exclude_paths,
            extensions,
            rules,
        }
    }

    pub fn appliest_to_ext(&self, ext: &FileExtension) -> bool {
        return self.extensions.contains(ext);
    }

    pub fn get_exclude_paths(&self) -> Vec<Regex> {
        return self.exclude_paths.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_matches(&self, text: &str) -> Vec<&ScoutRule> {
        let matches: Vec<&ScoutRule> = self
            .rules
            .iter()
            .filter(|rule| rule.matches(text))
            .collect();
        return matches;
    }
}
#[derive(Debug)]
pub struct ScoutRule {
    description: String,
    id: String,
    pattern: Regex,
}

impl ScoutRule {
    pub fn new(id: String, description: String, pattern: Regex) -> ScoutRule {
        ScoutRule {
            description,
            id,
            pattern,
        }
    }

    pub fn matches(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }
}
