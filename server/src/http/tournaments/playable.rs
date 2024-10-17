use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::tournament::manager::TournamentManager;

pub async fn playable_tournament() -> Result<Json<Value>, StatusCode> {
    let manager = TournamentManager::build();
    if manager.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    let manager = manager.unwrap();
    let tournament = manager.get_playable_tournament();
    if tournament.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    Ok(Json(json!(tournament.unwrap())))
}