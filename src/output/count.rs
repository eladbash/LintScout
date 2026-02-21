use crate::scanner::ScanResult;

pub fn format(result: &ScanResult) -> String {
    result.stats.findings_count.to_string()
}
