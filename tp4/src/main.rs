use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::{Semaphore};
use tp4::{
    http::http_status_code::HttpStatusCode, response::pi::leibniz, server::web_server::WebServer,
};

struct AppState {
    stats: Arc<RwLock<Statistics>>,
    upload_semaphore: Arc<Semaphore>,
}

struct Statistics {
    total_exceptions: usize,
    files_processed: usize,
    per_file: HashMap<String, usize>,
}

fn main() {
    let state = AppState {
        stats: Arc::new(RwLock::new(Statistics {
            total_exceptions: 0,
            files_processed: 0,
            per_file: HashMap::new(),
        })),
        upload_semaphore: Arc::new(Semaphore::new(4)),
    };



    let mut server = WebServer::new();
    server.get("/", |context| {
        context.set_status(HttpStatusCode::Ok);
        context.send_text("Hello World");
    });

    server.post("/upload", |context| {
        let permit = match state.upload_semaphore.clone().try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => {
                context.set_status(HttpStatusCode::TooManyRequests);
                context.send_text("Too many files being processed");
                return;
            }
        };

        let maybe_file = context.get_request().get_file("file");
        if maybe_file.is_none() || maybe_file.unwrap().is_empty() {
            context.set_status(HttpStatusCode::BadRequest);
            context.send_text("File not found or empty");
            return;
        }

        let file = maybe_file.unwrap();
        let filename = file.get_filename(); // You’ll need to implement this
        let content = file.get_content();   // Same here

        let exception_count = content
            .lines()
            .filter(|line| line.to_lowercase().contains("exception"))
            .count();

        {
            let mut stats = state.stats.write().unwrap();
            stats.total_exceptions += exception_count;
            stats.files_processed += 1;
            stats.per_file.insert(filename.clone(), exception_count);
        }

        context.set_status(HttpStatusCode::Ok);
        context.send_text(&format!("Processed file: {}", filename));

        drop(permit); // Explicit, though RAII will auto-release
    });

    server.get("/stats", move |context| {
        let stats = state.stats.read().unwrap();
        let response = format!(
            "Total exceptions: {}\nFiles processed: {}\nPer file: {:?}",
            stats.total_exceptions,
            stats.files_processed,
            stats.per_file
        );
        context.set_status(HttpStatusCode::Ok);
        context.send_text(&response);
    });

    server.threads(16);
    server.serve(5000);
}
