mod upload_handler;
mod stats_handler;
mod rejection_handler;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Semaphore, RwLock};
use warp::Filter;
use upload_handler::handle_upload;
use stats_handler::handle_stats;
use rejection_handler::handle_rejection;

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

    let upload_state = state.clone();
    let upload_semaphore = semaphore.clone();

    let upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(move |form| {
            let state = upload_state.clone();
            let sem = upload_semaphore.clone();
            handle_upload(form, state, sem)
        });

    let stats_route = warp::get()
        .and(warp::path("stats"))
        .and_then(move || handle_stats(state.clone()));

    let routes = upload_route.or(stats_route).recover(handle_rejection);

    println!("Server running at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
