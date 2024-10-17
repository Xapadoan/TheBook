use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{
    equipment::protection::Protection,
    inventory::{HasMutableInventory, Item, MutableItems},
    player::Player,
    unique_entity::UniqueEntity,
    warrior::{body::body_part::BodyPartKind, Warrior},
};
use uuid::Uuid;

use crate::{
    player::warriors::ReplaceProtection,
    repository::{PlayerRepository, Repository}, warrior::WarriorManager,
};

pub async fn replace_protection(
    Extension(mut player): Extension<Player>,
    Extension(mut warrior): Extension<Warrior>,
    Json((body_part_kind, inventory_slot)): Json<(BodyPartKind, Uuid)>,
) -> Result<Json<Value>, StatusCode> {
    let new_protection = take_protection_from_player(&mut player, &inventory_slot);
    if let None = new_protection { return Err(StatusCode::NOT_FOUND); }
    let new_protection = new_protection.unwrap();
    let protection_to_store = warrior.replace_protection(&body_part_kind, new_protection);
    if let Some(protection) = protection_to_store {
        player.inventory_mut().add_item(Item::Protection(protection));
    }
    let player_repo = PlayerRepository::build();
    if player_repo.is_err() { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    let player_repo = player_repo.unwrap();
    if player_repo.update(player.uuid(), &player).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    let warrior_manager = WarriorManager::build();
    if warrior_manager.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let warrior_manager = warrior_manager.unwrap();
    if warrior_manager.save(&warrior).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(json!(())))
}

fn take_protection_from_player(player: &mut Player, inventory_slot: &Uuid) -> Option<Protection> {
    let new_protection = player.inventory_mut().remove_item(inventory_slot);
    if let None = new_protection {
        eprintln!(
            "[WARN] Item {} not found for player {}",
            inventory_slot,
            player.uuid(),
        );
        return None;
    }
    let new_protection = Protection::try_from(new_protection.unwrap());
    if let Err(e) = new_protection {
        eprintln!(
            "[WARN] Item {} couldn't be converted to a protection:\n{e}",
            inventory_slot,
        );
        return None;
    }
    Some(new_protection.unwrap())
}
