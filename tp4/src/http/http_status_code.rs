#[derive(Clone, Copy)]
pub enum HttpStatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    TooManyRequests = 429,
    InternalServerError = 500,
}
