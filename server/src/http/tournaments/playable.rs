use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::{http::app::AppState, repository::sql_repository::tournaments::TournamentsRepository, tournament::manager::TournamentManager};

pub async fn playable_tournament(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let repo = TournamentsRepository::new(state.db_pool());
    let manager = TournamentManager::new(repo);
    let tournament = manager.get_playable_tournament().await;
    if tournament.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    Ok(Json(json!(tournament.unwrap())))
}