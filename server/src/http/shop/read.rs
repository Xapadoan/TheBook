use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::shop;

pub async fn read_shop() -> Result<Json<Value>, StatusCode> {
    match shop::read_shop() {
        Ok(shop) => Ok(Json(json!(shop))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}