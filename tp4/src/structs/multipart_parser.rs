pub struct MultipartParser;

impl MultipartParser {
    /// Extrae el boundary desde el Content-Type
    pub fn extract_boundary(content_type: &str) -> Option<String> {
        content_type
            .split("boundary=")
            .nth(1)
            .map(|s| format!("--{}", s.trim()))
    }

    /// Parsea el body multipart y cuenta las líneas que contienen "exception"
    pub fn parse_file_and_count_exceptions(
        body: &str,
        boundary: &str,
    ) -> Result<(String, u64), String> {
        let parts: Vec<&str> = body.split(boundary).collect();

        for part in parts {
            if part.contains("Content-Disposition") && part.contains("name=\"file\"") {
                let filename = part
                    .split("filename=")
                    .nth(1)
                    .and_then(|s| s.split('"').nth(1))
                    .unwrap_or("unknown.txt")
                    .to_string();

                let content = part
                    .split("\r\n\r\n")
                    .nth(1)
                    .unwrap_or("")
                    .trim_end_matches("--")
                    .trim();

                let count = content
                    .lines()
                    .filter(|line| line.to_lowercase().contains("exception"))
                    .count() as u64;

                return Ok((filename, count));
            }
        }

        Err("No file part found".to_string())
    }
}
