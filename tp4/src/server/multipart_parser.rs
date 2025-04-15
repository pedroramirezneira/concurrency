use std::collections::HashMap;

#[derive(Debug)]
pub struct MultipartField {
    pub name: String,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub value: Vec<u8>,
}

pub fn parse_multipart_body(body: &str, boundary: &str) -> HashMap<String, MultipartField> {
    let mut fields = HashMap::new();
    let delimiter = format!("--{}", boundary);
    let end_delimiter = format!("--{}--", boundary);

    for part in body.split(&delimiter) {
        let part = part.trim_matches(|c| c == '\r' || c == '\n');

        if part.is_empty() || part == "--" || part == end_delimiter {
            continue;
        }

        let sections: Vec<&str> = part.splitn(2, "\r\n\r\n").collect();
        if sections.len() < 2 {
            continue;
        }

        let headers = sections[0];
        let body = sections[1];

        let mut name = None;
        let mut filename = None;
        let mut content_type = None;

        for line in headers.lines() {
            if line.to_lowercase().starts_with("content-disposition") {
                for piece in line.split(';') {
                    let piece = piece.trim();
                    if piece.starts_with("name=") {
                        name = Some(
                            piece
                                .trim_start_matches("name=")
                                .trim_matches('"')
                                .to_string(),
                        );
                    } else if piece.starts_with("filename=") {
                        filename = Some(
                            piece
                                .trim_start_matches("filename=")
                                .trim_matches('"')
                                .to_string(),
                        );
                    }
                }
            } else if line.to_lowercase().starts_with("content-type:") {
                content_type = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
            }
        }

        if let Some(field_name) = name {
            fields.insert(
                field_name.clone(),
                MultipartField {
                    name: field_name,
                    filename,
                    content_type,
                    value: body.as_bytes().to_vec(),
                },
            );
        }
    }

    fields
}
