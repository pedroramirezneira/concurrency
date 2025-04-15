use std::collections::HashMap;

pub struct Request {
    params: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn new(params: HashMap<String, String>) -> Request {
        Request { params, body: None }
    }
    
    pub fn new_with_body(params: HashMap<String, String>, body: Option<String>) -> Request {
        Request { params, body }
    }

    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn get_body(&self) -> Option<&String> {
        self.body.as_ref()
    }
}
