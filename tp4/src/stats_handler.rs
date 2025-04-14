use std::collections::HashMap;
use std::convert::Infallible;
use serde::Serialize;
use crate::SharedState;

#[derive(Serialize)]
struct Stats {
    total_exceptions: usize,
    files_processed: usize,
    per_file: HashMap<String, usize>,
}

pub(crate) async fn handle_stats(state: SharedState) -> Result<impl warp::Reply, Infallible> {
    let state_lock = state.read().await;
    let response = Stats {
        total_exceptions: state_lock.total_exceptions,
        files_processed: state_lock.files_processed,
        per_file: state_lock.per_file.clone(),
    };
    Ok(warp::reply::json(&response))
}
