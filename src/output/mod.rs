pub mod count;
pub mod json;
pub mod sarif;
pub mod text;

use crate::scanner::ScanResult;

pub fn format_output(result: &ScanResult, format: &str) -> String {
    match format {
        "json" => json::format(result),
        "count" => count::format(result),
        "sarif" => sarif::format(result),
        _ => text::format(result),
    }
}
