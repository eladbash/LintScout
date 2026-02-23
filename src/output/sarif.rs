use std::collections::HashMap;

use serde::Serialize;

use crate::scanner::ScanResult;

#[derive(Serialize)]
struct SarifReport {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<SarifRun>,
}

#[derive(Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
struct SarifDriver {
    name: &'static str,
    version: &'static str,
    rules: Vec<SarifRuleDescriptor>,
}

#[derive(Serialize)]
struct SarifRuleDescriptor {
    id: String,
    #[serde(rename = "shortDescription")]
    short_description: SarifMessage,
}

#[derive(Serialize)]
struct SarifResult {
    #[serde(rename = "ruleId")]
    rule_id: String,
    #[serde(rename = "ruleIndex")]
    rule_index: usize,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    properties: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Serialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct SarifRegion {
    #[serde(rename = "startLine")]
    start_line: usize,
}

pub fn format(result: &ScanResult) -> String {
    let mut rule_map: HashMap<String, usize> = HashMap::new();
    let mut rules: Vec<SarifRuleDescriptor> = Vec::new();

    for finding in &result.findings {
        let composite_key = format!("{}/{}", finding.linter, finding.rule_id);
        if !rule_map.contains_key(&composite_key) {
            let index = rules.len();
            rule_map.insert(composite_key.clone(), index);
            rules.push(SarifRuleDescriptor {
                id: composite_key,
                short_description: SarifMessage {
                    text: finding.rule_description.clone(),
                },
            });
        }
    }

    let results: Vec<SarifResult> = result
        .findings
        .iter()
        .map(|f| {
            let composite_key = format!("{}/{}", f.linter, f.rule_id);
            let rule_index = rule_map[&composite_key];

            let mut properties = HashMap::new();
            properties.insert(
                "linter".to_string(),
                serde_json::Value::String(f.linter.clone()),
            );
            properties.insert(
                "scout_name".to_string(),
                serde_json::Value::String(f.scout_name.clone()),
            );
            if let Some(ref suppressed) = f.suppressed_rules {
                properties.insert(
                    "suppressed_rules".to_string(),
                    serde_json::json!(suppressed),
                );
            }

            SarifResult {
                rule_id: composite_key,
                rule_index,
                message: SarifMessage {
                    text: format!(
                        "{}: {} (line: {})",
                        f.rule_description,
                        f.line_text.trim(),
                        f.line_number
                    ),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: f.path.clone(),
                        },
                        region: SarifRegion {
                            start_line: f.line_number,
                        },
                    },
                }],
                properties,
            }
        })
        .collect();

    let report = SarifReport {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json",
        version: "2.1.0",
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "lintscout",
                    version: env!("CARGO_PKG_VERSION"),
                    rules,
                },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&report).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::Finding;
    use crate::stats::ScanStats;

    #[test]
    fn sarif_output_structure() {
        let result = ScanResult {
            findings: vec![Finding {
                path: "test.js".into(),
                line_number: 10,
                line_text: "// eslint-disable-next-line no-alert".into(),
                scout_name: "eslint".into(),
                linter: "eslint".into(),
                rule_id: "eslint-disable-next-line".into(),
                rule_description: "ESLint disable next line".into(),
                suppressed_rules: Some(vec!["no-alert".into()]),
            }],
            stats: ScanStats::default(),
            errors: Vec::new(),
        };

        let output = format(&result);
        let json: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");

        assert_eq!(json["version"], "2.1.0");
        assert_eq!(json["runs"][0]["tool"]["driver"]["name"], "lintscout");

        let rules = json["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0]["id"], "eslint/eslint-disable-next-line");

        let results = json["runs"][0]["results"].as_array().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["ruleIndex"], 0);
        assert_eq!(
            results[0]["locations"][0]["physicalLocation"]["region"]["startLine"],
            10
        );
        assert_eq!(results[0]["properties"]["suppressed_rules"][0], "no-alert");
    }

    #[test]
    fn sarif_deduplicates_rules() {
        let result = ScanResult {
            findings: vec![
                Finding {
                    path: "a.js".into(),
                    line_number: 1,
                    line_text: "// eslint-disable-next-line".into(),
                    scout_name: "eslint".into(),
                    linter: "eslint".into(),
                    rule_id: "eslint-disable-next-line".into(),
                    rule_description: "ESLint disable next line".into(),
                    suppressed_rules: None,
                },
                Finding {
                    path: "b.js".into(),
                    line_number: 5,
                    line_text: "// eslint-disable-next-line".into(),
                    scout_name: "eslint".into(),
                    linter: "eslint".into(),
                    rule_id: "eslint-disable-next-line".into(),
                    rule_description: "ESLint disable next line".into(),
                    suppressed_rules: None,
                },
            ],
            stats: ScanStats::default(),
            errors: Vec::new(),
        };

        let output = format(&result);
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();
        let rules = json["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap();
        assert_eq!(rules.len(), 1);
        let results = json["runs"][0]["results"].as_array().unwrap();
        assert_eq!(results.len(), 2);
    }
}
