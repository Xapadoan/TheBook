use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{
    player::Player,
    tournament::Tournament,
    unique_entity::UniqueEntity,
};
use uuid::Uuid;

use crate::{
    http::app::AppState,
    player::PlayerManager,
    repository::sql_repository::tournaments_warriors::TournamentsWarriorsRepository,
    tournament::ContestantsRegisterer,
};

pub async fn register_warriors(
    Extension(player): Extension<Player>,
    Extension(mut tournament): Extension<Tournament>,
    State(state): State<Arc<AppState>>,
    Json(warriors_uuids): Json<Vec<Uuid>>,
) -> Result<Json<Value>, StatusCode> {
    let manager = PlayerManager::new(&player);
    let repo = TournamentsWarriorsRepository::new(state.db_pool());
    let registerer = ContestantsRegisterer::new(repo);
    for warrior_uuid in warriors_uuids {
        if let Some(warrior) = manager.read_warrior(&warrior_uuid) {
            let mut warrior = warrior.clone();
            if registerer.register_contestant(player.uuid(), &mut tournament, &mut warrior).await.is_err() {
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    Ok(Json(json!(())))
}