use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

use crate::player;

#[axum::debug_handler]
pub async fn sell_item(
    Extension(mut player): Extension<Player>,
    Path(slot_uuid): Path<uuid::Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let gold_gained = player::sell_item(&mut player, &slot_uuid);
    if gold_gained.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let gold_gained = gold_gained.unwrap();
    match gold_gained {
        Some(_) => Ok(Json(json!(player))),
        None => Err(StatusCode::NOT_FOUND)
    }
}