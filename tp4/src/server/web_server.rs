use crate::{http::{http_method::{http_method_from_string, HttpMethod}, http_status_code::HttpStatusCode}, response::not_found_response::not_found_response, server::{combinations::generate_route_combinations, request::Request}};

use std::{
    collections::HashMap,
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread::{self},
};

use super::{context::Context, pair::Pair};

pub struct WebServer {
    handlers: HashMap<Pair<HttpMethod, String>, fn(context: &mut Context)>,
    threads: u32,
}

impl WebServer {
    pub fn new() -> WebServer {
        WebServer {
            handlers: HashMap::new(),
            threads: 10,
        }
    }

    pub fn threads(&mut self, threads: u32) {
        self.threads = threads;
    }

    pub fn get(&mut self, route: &str, handler: fn(context: &mut Context)) {
        let key = Pair::new(HttpMethod::Get, route.to_string());
        self.handlers.insert(key, handler);
    }

    pub fn post(&mut self, route: &str, handler: fn(context: &mut Context)) {
        let key = Pair::new(HttpMethod::Post, route.to_string());
        self.handlers.insert(key, handler);
    }

    pub fn serve(self, port: u32) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address)
            .unwrap_or_else(|_| panic!("Failed to bind to port {}", port));
        println!("Server is listening on port {}", port);

        let server = Arc::new(self);
        let (tx, rx) = mpsc::channel::<Result<TcpStream, Error>>();
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..server.threads {
            let server = Arc::clone(&server);
            let rx = Arc::clone(&rx);
            thread::spawn(move || {
                loop {
                    let lock = rx.try_lock();
                    if lock.is_err() {
                        drop(lock);
                        continue;
                    }

                    let incoming = lock.as_ref().unwrap().try_recv();
                    drop(lock);
                    if incoming.is_err() {
                        continue;
                    }

                    let mut stream = incoming.unwrap().unwrap();
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

                    let parts = request.split("\r\n\r\n").collect::<Vec<&str>>();
                    let headers = parts[0];
                    let body = if parts.len() > 1 {
                        parts[1].to_string()
                    } else {
                        "".to_string()
                    };

                    let mut content_type_header = None;
                    for line in headers.lines() {
                        if line.to_lowercase().starts_with("content-type:") {
                            content_type_header = Some(line.to_string());
                            break;
                        }
                    }

                    let mut boundary = None;
                    if let Some(header) = content_type_header {
                        if header.contains("multipart/form-data") {
                            let parts = header.split(';').collect::<Vec<&str>>();
                            for part in parts {
                                if part.trim().starts_with("boundary=") {
                                    boundary = Some(
                                        part.trim()
                                            .trim_start_matches("boundary=")
                                            .trim_matches('"')
                                            .to_string(),
                                    );
                                }
                            }
                        }
                    }

                    let args = headers.split("\r\n").collect::<Vec<&str>>();
                    let method_str = args[0].split(" ").collect::<Vec<&str>>()[0];
                    let method = http_method_from_string(method_str);
                    if method.is_none() {
                        return;
                    }

                    let route = args[0].split(" ").collect::<Vec<&str>>()[1];
                    let possible_routes = generate_route_combinations(route);
                    let formatted_route = possible_routes.iter().find(|route| {
                        server
                            .handlers
                            .contains_key(&Pair::new(method.clone().unwrap(), route.to_string()))
                    });

                    if formatted_route.is_none() {
                        stream.write_all(not_found_response().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }

                    let formatted_route = formatted_route.unwrap();
                    let key = Pair::new(method.unwrap(), formatted_route.to_string());
                    let handler = server.handlers.get(&key).unwrap();

                    let mut params = HashMap::<String, String>::new();
                    if let Some(index) = formatted_route.split('/').position(|path| path == ":a") {
                        if let Some(param) = route.split('/').collect::<Vec<&str>>().get(index) {
                            params.insert("a".to_string(), param.to_string());
                        }
                    }

                    let request = Request::new_with_body(params, Some(body));
                    let mut context = Context::new(request, HttpStatusCode::Ok);

                    if boundary.is_some() {
                        context.set_boundary(boundary.unwrap());
                    }

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
            });
        }

        for incoming in listener.incoming() {
            let _ = tx.send(incoming);
        }
    }
}
