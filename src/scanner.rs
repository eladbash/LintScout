use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use serde::Serialize;

use crate::error::Result;
use crate::finding::Finding;
use crate::scout::Scout;
use crate::stats::ScanStats;

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub findings: Vec<Finding>,
    pub stats: ScanStats,
    #[serde(skip)]
    pub errors: Vec<String>,
}

pub struct Scanner {
    root_path: PathBuf,
    scouts: Vec<Scout>,
    exclude_patterns: Vec<String>,
    respect_gitignore: bool,
}

impl Scanner {
    pub fn new(path: impl Into<PathBuf>, scouts: Vec<Scout>) -> Self {
        Scanner {
            root_path: path.into(),
            scouts,
            exclude_patterns: Vec::new(),
            respect_gitignore: true,
        }
    }

    pub fn with_excludes(mut self, excludes: Vec<String>) -> Self {
        self.exclude_patterns = excludes;
        self
    }

    pub fn with_gitignore(mut self, respect: bool) -> Self {
        self.respect_gitignore = respect;
        self
    }

    pub fn run(&self) -> Result<ScanResult> {
        let start = Instant::now();
        let mut stats = ScanStats::default();
        let mut findings = Vec::new();
        let mut errors = Vec::new();

        let mut builder = WalkBuilder::new(&self.root_path);
        builder.git_ignore(self.respect_gitignore);

        if !self.exclude_patterns.is_empty() {
            let mut overrides = OverrideBuilder::new(&self.root_path);
            for pattern in &self.exclude_patterns {
                let neg = format!("!{pattern}");
                if let Err(e) = overrides.add(&neg) {
                    errors.push(format!("invalid exclude pattern '{pattern}': {e}"));
                    stats.errors_count += 1;
                }
            }
            if let Ok(built) = overrides.build() {
                builder.overrides(built);
            }
        }

        for entry in builder.build() {
            stats.files_walked += 1;

            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    errors.push(format!("walk error: {e}"));
                    stats.errors_count += 1;
                    continue;
                }
            };

            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let matching_scouts: Vec<&Scout> = self
                .scouts
                .iter()
                .filter(|s| s.applies_to_file(path))
                .collect();

            if matching_scouts.is_empty() {
                stats.files_skipped += 1;
                continue;
            }

            stats.files_scanned += 1;

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => {
                    errors.push(format!("{}: {}", path.display(), e));
                    stats.errors_count += 1;
                    stats.files_skipped += 1;
                    continue;
                }
            };

            let file_path = path.display().to_string();

            for (line_number, line_text) in content.lines().enumerate() {
                let line_number = line_number + 1;
                for scout in &matching_scouts {
                    for rule in scout.find_matches(line_text) {
                        findings.push(Finding {
                            path: file_path.clone(),
                            line_number,
                            line_text: line_text.to_string(),
                            scout_name: scout.name.clone(),
                            linter: scout.linter.clone(),
                            rule_id: rule.id.clone(),
                            rule_description: rule.description.clone(),
                        });
                        stats.findings_count += 1;
                    }
                }
            }
        }

        stats.duration_ms = start.elapsed().as_millis() as u64;

        Ok(ScanResult {
            findings,
            stats,
            errors,
        })
    }
}
