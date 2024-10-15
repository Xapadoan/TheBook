use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

use crate::replay::available_replays;

#[axum::debug_handler]
pub async fn new_replays(
    Extension(player): Extension<Player>
) -> Result<Json<Value>, StatusCode> {
    match available_replays(&player) {
        Ok(map) => Ok(Json(json!(map))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
