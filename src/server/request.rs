use std::collections::HashMap;

pub struct Request {
    params: HashMap<String, String>,
}

impl Request {
    pub fn new(params: HashMap<String, String>) -> Request {
        Request { params }
    }

    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}
