use std::convert::Infallible;

use axum::{Extension, Json};
use serde_json::{json, Value};
use shared::tournament::Tournament;

#[axum::debug_handler]
pub async fn read_replay(
    Extension(replay): Extension<Tournament>,
) -> Result<Json<Value>, Infallible> {
    Ok(Json(json!(replay)))
}