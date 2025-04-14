use std::convert::Infallible;

pub(crate) async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let message = "Valid routes:\nPOST /upload - Upload a file for analysis\nGET /stats - Show statistics";
    Ok(warp::reply::with_status(
        message,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}
