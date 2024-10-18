use std::convert::Infallible;

use axum::{Extension, Json};
use serde_json::{json, Value};
use shared::warrior::Warrior;

pub async fn read_warrior(
    Extension(warrior): Extension<Warrior>,
) -> Result<Json<Value>, Infallible> {
    Ok(Json(json!(warrior)))
}