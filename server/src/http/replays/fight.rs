use axum::{extract::Query, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{replay::FightReplaySummary, tournament::Tournament, unique_entity::UniqueEntity};

use crate::replay::ReplayManager;

#[axum::debug_handler]
pub async fn read_fight_replay(
    Extension(replay): Extension<Tournament>,
    Query(fight_summary): Query<FightReplaySummary>,
) -> Result<Json<Value>, StatusCode> {
    let manager = ReplayManager::new(replay.uuid());
    let replay = manager.get_fight_replay(&fight_summary);
    if replay.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let warriors = manager.get_fight_warriors(&fight_summary);
    if warriors.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(json!((replay.unwrap(), warriors.unwrap()))))
}