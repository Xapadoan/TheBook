use std::path::PathBuf;

use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::{json, Value};
use shared::equipment::weapon::OptionalMutableWeapon;
use shared::inventory::{HasMutableInventory, Item, MutableItems};
use shared::player::Player;
use shared::unique_entity::UniqueEntity;
use shared::warrior::body::body_part::PROTECTABLE_BODY_PARTS;
use shared::warrior::{MutableWarriorCollection, Warrior};

use crate::player::warriors::TakeProtections;
use crate::repository::{FileRepository, PlayerRepository, Repository};

pub async fn remove_warrior(
    Extension(mut player): Extension<Player>,
    Extension(mut warrior): Extension<Warrior>,
) -> Result<Json<Value>, StatusCode> {
    let protections = warrior.take_protections(PROTECTABLE_BODY_PARTS.iter().collect());
    for protection in protections {
        player.inventory_mut().add_item(Item::Protection(protection));
    }
    if let Some(weapon) = warrior.weapon_mut().take() {
        player.inventory_mut().add_item(Item::Weapon(weapon));
    }
    player.take_warrior(warrior.uuid());
    let warrior_repo = FileRepository::build(PathBuf::from("saves/warriors"));
    if warrior_repo.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    let warrior_repo: FileRepository<Warrior> = warrior_repo.unwrap();
    if warrior_repo.delete(warrior.uuid()).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let player_repo = PlayerRepository::build();
    if player_repo.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    let player_repo = player_repo.unwrap();
    if player_repo.update(player.uuid(), &player).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(json!(())))
}
