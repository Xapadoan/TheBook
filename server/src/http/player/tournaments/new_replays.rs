use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;

use crate::{http::app::AppState, replay::ReplayManager, repository::sql_repository::tournaments::TournamentsRepository, tournament::manager::{self, TournamentManager}};

#[axum::debug_handler]
pub async fn new_replays(
    Extension(player): Extension<Player>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let repo = TournamentsRepository::new(state.db_pool());
    let manager = TournamentManager::new(repo);
    match ReplayManager::map_warriors_to_replays(&manager, &player).await {
        Ok(map) => Ok(Json(json!(map))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
