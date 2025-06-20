use crate::{
    http::{
        http_method::{HttpMethod, http_method_from_string},
        http_status_code::HttpStatusCode,
    },
    response::not_found_response::not_found_response,
    server::{combinations::generate_route_combinations, request::Request},
};
use std::{
    collections::HashMap,
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread::{self},
};

use super::{context::Context, pair::Pair};

pub struct WebServer {
    handlers: HashMap<Pair<HttpMethod, String>, Box<dyn Fn(&mut Context) + Send + Sync>>,
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

    /** Adds a get route to the server */
    pub fn get(&mut self, route: &str, handler: Box<dyn Fn(&mut Context) + Send + Sync>) {
        let key = Pair::new(HttpMethod::Get, route.to_string());
        self.handlers.insert(key, handler);
    }

    pub fn post(&mut self, route: &str, handler: Box<dyn Fn(&mut Context) + Send + Sync>) {
        let key = Pair::new(HttpMethod::Post, route.to_string());
        self.handlers.insert(key, handler);
    }

    pub fn serve(self, port: u32) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).expect("Failed to bind to port");
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
                    if incoming.is_err() || incoming.as_ref().unwrap().is_err() {
                        continue;
                    }

                    let mut stream = incoming.unwrap().unwrap();

                    let mut buffer = Vec::new();
                    let mut chunk = [0u8; 1024];
                    let mut headers_str = String::new();
                    let mut content_length = 0;
                    let mut headers_end_pos = 0;

                    loop {
                        let bytes_read = stream.read(&mut chunk).unwrap_or(0);
                        if bytes_read == 0 {
                            break;
                        }

                        buffer.extend_from_slice(&chunk[..bytes_read]);

                        if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                            headers_end_pos = pos + 4;
                            headers_str =
                                String::from_utf8_lossy(&buffer[..headers_end_pos]).to_string();

                            if let Some(cl_line) = headers_str
                                .lines()
                                .find(|l| l.to_ascii_lowercase().starts_with("content-length:"))
                            {
                                content_length = cl_line
                                    .split(":")
                                    .nth(1)
                                    .unwrap()
                                    .trim()
                                    .parse::<usize>()
                                    .unwrap_or(0);
                            }
                            break;
                        }
                    }

                    while buffer.len() < headers_end_pos + content_length {
                        let bytes_read = stream.read(&mut chunk).unwrap_or(0);
                        if bytes_read == 0 {
                            break;
                        }
                        buffer.extend_from_slice(&chunk[..bytes_read]);
                    }

                    let args = headers_str.split("\r\n").collect::<Vec<&str>>();
                    let method = args[0].split(" ").next().unwrap_or("").to_string();
                    let route = args[0].split(" ").nth(1).unwrap_or("").to_string();
                    let method_enum = http_method_from_string(&method);
                    if method_enum.is_none() {
                        continue;
                    }

                    let possible_routes = generate_route_combinations(&route);
                    let formatted_route = possible_routes.iter().find(|r| {
                        server
                            .handlers
                            .contains_key(&Pair::new(method_enum.clone().unwrap(), r.to_string()))
                    });

                    if formatted_route.is_none() {
                        stream.write_all(not_found_response().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        continue;
                    }

                    let mut params = HashMap::<String, String>::new();
                    let index = formatted_route
                        .unwrap()
                        .split("/")
                        .position(|path| path.starts_with(":"));
                    if let Some(idx) = index {
                        if let Some(val) = route.split("/").collect::<Vec<&str>>().get(idx) {
                            let key = formatted_route
                                .unwrap()
                                .split("/")
                                .nth(idx)
                                .unwrap()
                                .trim_start_matches(":");
                            params.insert(key.to_string(), val.to_string());
                        }
                    }

                    let mut headers = HashMap::<String, String>::new();
                    for line in args.iter().skip(1) {
                        if let Some((k, v)) = line.split_once(": ") {
                            headers.insert(k.to_ascii_lowercase(), v.to_string());
                        }
                    }

                    let body = String::from_utf8_lossy(&buffer[headers_end_pos..]).to_string();
                    let request = Request::new(params, headers, body);
                    let status_code = HttpStatusCode::Ok;
                    let mut context = Context::new(request, status_code);

                    let handler_key =
                        Pair::new(method_enum.unwrap(), formatted_route.unwrap().to_string());
                    let handler = server.handlers.get(&handler_key).unwrap();
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
