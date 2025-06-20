pub struct MultipartParser;

impl MultipartParser {
    pub fn extract_boundary(content_type: &str) -> Option<String> {
        content_type
            .split("boundary=")
            .nth(1)
            .map(|s| format!("--{}", s.trim()))
    }

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

    pub fn extract_filename(body: &str, boundary: &str) -> Option<String> {
        let boundary_marker = format!("--{}", boundary);
        for part in body.split(&boundary_marker) {
            if let Some(disposition_line) = part.lines().find(|l| l.contains("Content-Disposition"))
            {
                if let Some(start) = disposition_line.find("filename=\"") {
                    let rest = &disposition_line[start + 10..];
                    if let Some(end) = rest.find('"') {
                        return Some(rest[..end].to_string());
                    }
                }
            }
        }
        None
    }

    pub fn extract_file_content(body: &str, boundary: &str) -> Option<Vec<u8>> {
        let boundary_marker = format!("--{}", boundary);
        let parts: Vec<&str> = body.split(&boundary_marker).collect();
        for part in parts {
            if part.contains("Content-Disposition") && part.contains("filename=") {
                // Buscamos el separador entre headers y contenido
                if let Some(idx) = part.find("\r\n\r\n") {
                    let content = &part[idx + 4..];
                    // Quitamos el trailing \r\n si está
                    let content = content.trim_end_matches("\r\n");
                    return Some(content.as_bytes().to_vec());
                }
            }
        }

        None
    }
}
