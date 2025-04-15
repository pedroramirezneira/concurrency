use crate::http::http_status_code::HttpStatusCode;

pub fn not_found_response() -> String {
    format!(
        "HTTP/1.1 {} Not Found\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: 9\r\n\
        Cache-Control: no-store, no-cache, must-revalidate\r\n\
        Connection: close\r\n\r\n\
        Not Found",
        HttpStatusCode::NotFound as u16
    )
}
