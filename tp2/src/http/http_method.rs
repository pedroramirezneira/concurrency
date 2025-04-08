#[derive(Eq, Hash, PartialEq, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
}

pub fn http_method_from_string(method: &str) -> Option<HttpMethod> {
    match method {
        "GET" => Some(HttpMethod::Get),
        "POST" => Some(HttpMethod::Post),
        "PUT" => Some(HttpMethod::Put),
        "DELETE" => Some(HttpMethod::Delete),
        "PATCH" => Some(HttpMethod::Patch),
        "HEAD" => Some(HttpMethod::Head),
        "OPTIONS" => Some(HttpMethod::Options),
        "CONNECT" => Some(HttpMethod::Connect),
        "TRACE" => Some(HttpMethod::Trace),
        _ => None,
    }
}
