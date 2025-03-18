use crate::{
    http::{
        http_method::{http_method_from_string, HttpMethod},
        http_status_code::HttpStatusCode,
    },
    response::not_found_response::not_found_response,
    server::request::Request,
};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use super::{context::Context, pair::Pair};

pub struct WebServer {
    handlers: HashMap<Pair<HttpMethod, String>, fn(context: &mut Context)>,
}

impl WebServer {
    pub fn new() -> WebServer {
        WebServer {
            handlers: HashMap::new(),
        }
    }

    /** Adds a get route to the server */
    pub fn get(&mut self, route: &str, handler: fn(context: &mut Context)) {
        let key = Pair::new(HttpMethod::Get, route.to_string());
        self.handlers.insert(key, handler);
    }

    pub fn serve(&self, port: u32) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address);
        if listener.is_err() {
            panic!("Failed to bind to port {}", port);
        }
        let listener = listener.unwrap();
        for incoming in listener.incoming() {
            if incoming.is_err() {
                continue;
            }
            let mut stream = incoming.unwrap();
            let mut buffer = vec![0; 1024 * 1024];
            let mut request = String::new();
            loop {
                let bytes_read = stream.read(&mut buffer).unwrap_or(0);
                if bytes_read == 0 {
                    break;
                }
                request.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
                if request.contains("\r\n\r\n") {
                    break;
                }
            }
            let args = request.split("\r\n").collect::<Vec<&str>>();
            let method = args[0].split(" ").collect::<Vec<&str>>()[0];
            let method = http_method_from_string(method);
            if method.is_none() {
                continue;
            }
            let route = args[0].split(" ").collect::<Vec<&str>>()[1];
            let key = Pair::new(method.unwrap(), route.to_string());
            let handler = self.handlers.get(&key);
            if handler.is_none() {
                stream.write_all(not_found_response().as_bytes()).unwrap();
                stream.flush().unwrap();
                continue;
            }
            let request = Request::new(HashMap::new());
            let status_code = HttpStatusCode::Ok;
            let mut context = Context::new(request, status_code);
            let handler = handler.unwrap();
            handler(&mut context);
            let body = context.get_body();
            let status_code = context.get_status();
            let response = format!(
                "HTTP/1.1 {} OK\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\
                Cache-Control: no-store, no-cache, must-revalidate\r\n\
                Connection: close\r\n\r\n\
                {}",
                (*status_code as u16).to_string(),
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
