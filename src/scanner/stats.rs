pub struct ScanStats {
    pub files_walked: u32,
    pub files_scanned: u32,
    pub findings_count: u32,
}

impl ScanStats {
    pub fn new() -> ScanStats {
        ScanStats {
            files_walked: 0,
            files_scanned: 0,
            findings_count: 0,
        }
    }

    pub fn files_walked_up(&mut self) {
        self.files_walked += 1;
    }

    pub fn files_scanned_up(&mut self) {
        self.files_scanned += 1;
    }

    pub fn findings_count_up(&mut self) {
        self.findings_count += 1;
    }

    pub fn get_files_scanned(&self) -> u32 {
        self.files_scanned
    }

    pub fn get_findings_count(&self) -> u32 {
        self.findings_count
    }
}
