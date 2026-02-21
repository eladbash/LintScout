use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct ScanStats {
    pub files_walked: u64,
    pub files_scanned: u64,
    pub files_skipped: u64,
    pub findings_count: u64,
    pub errors_count: u64,
    pub duration_ms: u64,
}
