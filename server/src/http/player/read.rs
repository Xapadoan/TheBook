use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

pub async fn read_player(
    Extension(player): Extension<Player>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!(player)))
}
