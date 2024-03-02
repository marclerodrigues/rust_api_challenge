use actix_web::{error, HttpRequest, HttpResponse};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    detail: String,
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();
    let payload = ErrorResponse {
        error: "Invalid Json".to_string(),
        detail,
    };
    let response = serde_json::to_string(&payload).unwrap();
    error::InternalError::from_response(err, HttpResponse::BadRequest().json(response)).into()
}
