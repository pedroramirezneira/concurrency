use warp::multipart::{FormData, Part};
use futures::StreamExt;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::SharedState;

pub(crate) async fn handle_upload(
    form: FormData,
    state: SharedState,
    sem: Arc<Semaphore>,
) -> Result<impl warp::Reply, Infallible> {
    let permit = match sem.clone().try_acquire_owned() {
        Ok(p) => p,
        Err(_) => {
            return Ok(warp::reply::with_status(
                "Too many files being processed",
                warp::http::StatusCode::TOO_MANY_REQUESTS,
            ));
        }
    };

    let parts: Vec<Part> = form.collect().await.unwrap_or_else(|_| vec![]);
    let file_part = parts.into_iter().find(|p| p.name() == "file");

    if let Some(part) = file_part {
        let filename = part.filename().map(String::from).unwrap_or("unknown.txt".into());
        let mut data = Vec::new();
        let mut stream = part.stream();

        while let Some(chunk) = stream.next().await {
            if let Ok(bytes) = chunk {
                data.extend(bytes);
            }
        }

        if data.is_empty() {
            return Ok(warp::reply::with_status(
                "File not found or empty",
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }

        let content = String::from_utf8_lossy(&data);
        let count = content
            .lines()
            .filter(|line| line.to_lowercase().contains("exception"))
            .count();

        let mut state_lock = state.write().await;
        state_lock.total_exceptions += count;
        state_lock.files_processed += 1;
        state_lock.per_file.insert(filename.clone(), count);
        drop(permit); // libera el semáforo

        Ok(warp::reply::with_status(
            format!("Processed file: {}", filename),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            "File not found or empty",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    }
}
