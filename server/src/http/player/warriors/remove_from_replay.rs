use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{tournament::contestant::TournamentContestant, unique_entity::UniqueEntity, warrior::Warrior};

use crate::{http::app::AppState, repository::{sql_repository::warriors::{UpdateWarriorSchema, WarriorsRepository}, RepositoryUpdate}};

pub async fn remove_warrior_from_replay(
    Extension(mut warrior): Extension<Warrior>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    warrior.set_current_tournament(None);
    let uuid = warrior.uuid().clone();
    let schema = UpdateWarriorSchema::from(warrior);
    let repo = WarriorsRepository::new(state.db_pool());
    if repo.update(&uuid, &schema).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    Ok(Json(json!(())))
}
