use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

use crate::player;

#[axum::debug_handler]
pub async fn buy_item(
    Extension(mut player): Extension<Player>,
    Path(slot_uuid): Path<uuid::Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let item = player::buy_item(&mut player, &slot_uuid);
    if item.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let item = item.unwrap();
    match item {
        Some(_) => Ok(Json(json!(item))),
        None => Err(StatusCode::NOT_FOUND)
    }
}
