use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::replay::tournament_replay;

#[axum::debug_handler]
pub async fn read_tournament_replay(
    Path(tournament_uuid): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    match tournament_replay(&tournament_uuid) {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(replay) => Ok(Json(json!(replay)))
    }
}