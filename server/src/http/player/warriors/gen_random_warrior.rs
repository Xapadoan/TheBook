use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;
use shared::unique_entity::UniqueEntity;
use shared::{random::Random, warrior::MutableWarriorCollection};
use shared::warrior::Warrior;

use crate::repository::{PlayerRepository, Repository};
use crate::warrior::WarriorManager;

pub async fn gen_random_warrior(
    Extension(mut player): Extension<Player>,
) -> Result<Json<Value>, StatusCode> {
    let warrior = Warrior::random();
    let warrior_manager = WarriorManager::build();
    if warrior_manager.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    let warrior_manager = warrior_manager.unwrap();
    if warrior_manager.create(&warrior).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    player.warriors_mut().push(warrior.clone());
    let player_repo = PlayerRepository::build();
    if player_repo.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    let player_repo = player_repo.unwrap();
    if player_repo.update(player.uuid(), &player).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(json!(warrior)))
}
