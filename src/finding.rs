use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub path: String,
    pub line_number: usize,
    pub line_text: String,
    pub scout_name: String,
    pub linter: String,
    pub rule_id: String,
    pub rule_description: String,
}
