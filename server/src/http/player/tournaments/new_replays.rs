use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

use crate::replay::ReplayManager;

#[axum::debug_handler]
pub async fn new_replays(
    Extension(player): Extension<Player>
) -> Result<Json<Value>, StatusCode> {
    match ReplayManager::map_warriors_to_replays(&player) {
        Ok(map) => Ok(Json(json!(map))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
