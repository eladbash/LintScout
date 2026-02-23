use crate::scanner::ScanResult;

pub fn format(result: &ScanResult) -> String {
    let mut out = String::new();

    if result.findings.is_empty() {
        out.push_str("No lint ignore directives found.\n");
    } else {
        for f in &result.findings {
            let suppressed = match &f.suppressed_rules {
                Some(rules) if !rules.is_empty() => {
                    format!(" (suppresses: {})", rules.join(", "))
                }
                _ => String::new(),
            };
            out.push_str(&format!(
                "{}:{} [{}:{}] {}{}\n    {}\n",
                f.path,
                f.line_number,
                f.linter,
                f.rule_id,
                f.rule_description,
                suppressed,
                f.line_text.trim()
            ));
        }
    }

    out.push('\n');
    out.push_str(&format!(
        "Files walked: {}, scanned: {}, skipped: {}\n",
        result.stats.files_walked, result.stats.files_scanned, result.stats.files_skipped
    ));
    out.push_str(&format!("Findings: {}\n", result.stats.findings_count));
    if result.stats.errors_count > 0 {
        out.push_str(&format!("Errors: {}\n", result.stats.errors_count));
    }
    out.push_str(&format!("Duration: {}ms\n", result.stats.duration_ms));

    out
}
