use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{
    player::Player,
    tournament::Tournament,
    unique_entity::UniqueEntity,
};
use uuid::Uuid;

use crate::{player::{register_contestant, PlayerManager}, tournament::manager::TournamentManager};

pub async fn register_warriors(
    Extension(player): Extension<Player>,
    Extension(mut tournament): Extension<Tournament>,
    Json(warriors_uuids): Json<Vec<Uuid>>,
) -> Result<Json<Value>, StatusCode> {
    let manager = PlayerManager::new(&player);
    let tournament_manager = TournamentManager::build();
    if tournament_manager.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    for warrior_uuid in warriors_uuids {
        if let Some(warrior) = manager.read_warrior(&warrior_uuid) {
            let mut warrior = warrior.clone();
            if register_contestant(player.uuid(), &mut tournament, &mut warrior).is_err() {
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    Ok(Json(json!(())))
}