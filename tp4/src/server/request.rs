use std::collections::HashMap;

pub struct Request {
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn new(
        params: HashMap<String, String>,
        headers: HashMap<String, String>,
        body: String,
    ) -> Request {
        Request {
            params,
            headers,
            body,
        }
    }

    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_ascii_lowercase())
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }
}
