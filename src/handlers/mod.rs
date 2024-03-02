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

pub fn conflict_error_handler() -> HttpResponse {
    let payload = ErrorResponse {
        error: "Item already exists".to_string(),
        detail: "You cannot create an item that already exists".to_string(),
    };
    let serialized_payload = serde_json::to_string(&payload).unwrap();

    HttpResponse::BadRequest().json(&serialized_payload).into()
}
