use grep::{SearchStrategy, search::ConcurrentSearch};
use std::{fs, sync::Arc, time::Instant};
use tp4::{
    http::http_status_code::HttpStatusCode,
    response::pi::leibniz,
    server::web_server::WebServer,
    structs::{multipart_parser::MultipartParser, shared_state::SharedState},
};

fn main() {
    let shared_state = Arc::new(SharedState::new());
    let mut server = WebServer::new();
    server.get(
        "/",
        Box::new(|context| {
            context.set_status(HttpStatusCode::Ok);
            context.send_text("Hello World");
        }),
    );
    server.get(
        "/pi/:a",
        Box::new(|context| {
            let start = Instant::now();
            let number = context.get_request().get_param("a").unwrap();
            let number = number.parse::<u64>();
            match number {
                Err(_) => {
                    context.set_status(HttpStatusCode::BadRequest);
                    context.send_text("Invalid number");
                }
                Ok(number) => {
                    context.set_status(HttpStatusCode::Ok);
                    let pi = leibniz(number);
                    let elapsed = start.elapsed().as_secs_f64();
                    let result = format!(
                        "Valor de pi para el termino {}: {} (Tiempo: {})",
                        number,
                        pi.to_string(),
                        &elapsed.to_string()
                    );
                    context.send_text(&result);
                }
            }
        }),
    );
    let state = Arc::clone(&shared_state);
    server.post(
        "/upload",
        Box::new(move |context| {
            let request_body = context.get_request().get_body();
            let content_type = context.get_request().get_header("Content-Type");

            let Some(content_type) = content_type else {
                context.set_status(HttpStatusCode::BadRequest);
                context.send_text("Missing Content-Type header");
                return;
            };

            let Some(boundary) = MultipartParser::extract_boundary(content_type) else {
                context.set_status(HttpStatusCode::BadRequest);
                context.send_text("Missing boundary in Content-Type");
                return;
            };

            let filename = MultipartParser::extract_filename(request_body, &boundary)
                .unwrap_or_else(|| "unknown".to_string());

            let Some(_permit) = state.try_start_processing() else {
                {
                    let mut stats = state.stats.write().unwrap();
                    stats.add_file(&filename, 1);
                }
                context.set_status(HttpStatusCode::TooManyRequests);
                context.send_text("Too many files being processed");
                return;
            };

            match MultipartParser::extract_file_content(request_body, &boundary) {
                Some(file_content) => {
                    // 1. Guardar archivo temporalmente
                    let temp_path = format!("/tmp/{}", filename);
                    if fs::write(&temp_path, &file_content).is_err() {
                        context.set_status(HttpStatusCode::InternalServerError);
                        context.send_text("Error writing temporary file");
                        return;
                    }
                    let searcher = ConcurrentSearch;
                    let count = searcher.search(&[temp_path.clone()], "exception");
                    {
                        let mut stats = state.stats.write().unwrap();
                        stats.add_file(&filename, count.try_into().unwrap());
                    }

                    let _ = fs::remove_file(&temp_path);

                    context.set_status(HttpStatusCode::Ok);
                    context.send_text(&format!(
                        "Processed file: {} with {} exceptions",
                        filename, count
                    ));
                }
                None => {
                    context.set_status(HttpStatusCode::BadRequest);
                    context.send_text("No file content found");
                }
            }
        }),
    );
    let state_for_stats = Arc::clone(&shared_state);
    server.get(
        "/stats",
        Box::new(move |context| {
            let stats = state_for_stats.stats.read().unwrap();
            context.set_status(HttpStatusCode::Ok);
            context.send_text(&stats.as_string());
        }),
    );
    server.threads(16);
    server.serve(5000);
}
