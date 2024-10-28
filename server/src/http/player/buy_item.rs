use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

use crate::{http::app::AppState, player, shop::ShopManager};

#[axum::debug_handler]
pub async fn buy_item(
    Extension(mut player): Extension<Player>,
    Path(slot_uuid): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let manager = ShopManager::new(state.db_pool());
    let item = player::buy_item(&manager, &mut player, &slot_uuid).await;
    if item.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let item = item.unwrap();
    match item {
        Some(_) => Ok(Json(json!(item))),
        None => Err(StatusCode::NOT_FOUND)
    }
}
