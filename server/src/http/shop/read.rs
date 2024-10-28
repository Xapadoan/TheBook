use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::{http::app::AppState, shop::ShopManager};

pub async fn read_shop(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let manager = ShopManager::new(state.db_pool());
    match manager.read_shop().await {
        Ok(shop) => Ok(Json(json!(shop))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}