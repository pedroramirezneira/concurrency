use std::collections::HashMap;

pub struct Stats {
    pub total_exceptions: u64,
    pub file_count: u64,
    pub per_file: HashMap<String, u64>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total_exceptions: 0,
            file_count: 0,
            per_file: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, filename: &str, exceptions: u64) {
        self.total_exceptions += exceptions;
        self.file_count += 1;
        *self.per_file.entry(filename.to_string()).or_insert(0) += exceptions;
    }

    pub fn as_string(&self) -> String {
        format!(
            "Total exceptions: {}\nFiles processed: {}\nPer file: {:?}",
            self.total_exceptions, self.file_count, self.per_file
        )
    }
}
