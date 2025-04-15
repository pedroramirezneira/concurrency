mod server;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, Semaphore};
use tp4::http::http_status_code::HttpStatusCode::BadRequest;
use tp4::server::{multipart_parser::parse_multipart_body, web_server::WebServer};

#[derive(Default)]
struct AppState {
    total_exceptions: usize,
    files_processed: usize,
    per_file: HashMap<String, usize>,
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(AppState::default()));
    let semaphore = Arc::new(Semaphore::new(4));

    let _upload_state = state.clone();
    let _upload_semaphore = semaphore.clone();
    let mut server = WebServer::new();
    server.post("/upload", |context| {
        let body = context.get_request().get_body();
        let boundary = context.get_boundary();
        if body.is_none() || boundary.is_none() {
            context.set_status(BadRequest);
            context.send_text("ERROR");
            return;
        }
        let _map = parse_multipart_body(body.unwrap(), boundary.unwrap());
        context.send_text("Jajajaja");
    });
    server.threads(16);
    server.serve(5000);
}
