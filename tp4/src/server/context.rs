use super::request::Request;
use crate::http::http_status_code::HttpStatusCode;

pub struct Context {
    request: Request,
    status: HttpStatusCode,
    completed: bool,
    body: String,
}

impl Context {
    pub fn new(request: Request, status: HttpStatusCode) -> Context {
        Context {
            request,
            status,
            completed: false,
            body: String::new(),
        }
    }

    /** Gets the request associated with the context. */
    pub fn get_request(&self) -> &Request {
        &self.request
    }

    /** Gets the status code of the response. */
    pub fn get_status(&self) -> &HttpStatusCode {
        &self.status
    }

    /** Sets the status code of the response. */
    pub fn set_status(&mut self, status: HttpStatusCode) {
        if self.completed {
            panic!("Cannot set status code after response has been sent");
        }
        self.status = status;
    }

    /** Sends a text response to the client, completing the context. */
    pub fn send_text(&mut self, text: &str) {
        if self.completed {
            panic!("Cannot send response after response has been sent");
        }
        self.body = text.to_string();
    }

    /** Gets the set response through send_text */
    pub fn get_body(&self) -> &String {
        &self.body
    }
}
