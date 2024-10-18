use axum::{http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::auth;

#[derive(Debug, Deserialize)]
pub struct SignUpPayload {
    username: String,
    display_name: String,
}

#[axum::debug_handler]
pub async fn signup(
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<Value>, StatusCode> {
    let session = auth::signup(payload.username, payload.display_name);
    if session.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(json!(session.unwrap())))
}