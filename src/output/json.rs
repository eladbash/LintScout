use crate::scanner::ScanResult;

pub fn format(result: &ScanResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}
