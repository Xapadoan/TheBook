use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{
    equipment::weapon::{OptionalMutableWeapon, Weapon},
    inventory::{HasMutableInventory, Item, MutableItems},
    player::Player,
    unique_entity::UniqueEntity,
    warrior::Warrior,
};
use uuid::Uuid;

use crate::{
    repository::{PlayerRepository, Repository},
    warrior::WarriorManager,
};

pub async fn replace_weapon(
    Extension(mut player): Extension<Player>,
    Extension(mut warrior): Extension<Warrior>,
    Json(inventory_slot): Json<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let new_weapon = take_weapon_from_player(&mut player, &inventory_slot);
    if let None = new_weapon { return Err(StatusCode::NOT_FOUND); }
    let new_weapon = new_weapon.unwrap();
    let old_weapon = warrior.replace_weapon(new_weapon);
    if let Some(weapon) = old_weapon {
        player.inventory_mut().add_item(Item::Weapon(weapon));
    }
    let player_repo = PlayerRepository::build();
    if player_repo.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let player_repo = player_repo.unwrap();
    if player_repo.update(player.uuid(), &player).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
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

fn take_weapon_from_player(player: &mut Player, inventory_slot: &Uuid) -> Option<Weapon> {
    let new_weapon = player.inventory_mut().remove_item(inventory_slot);
    if let None = new_weapon {
        eprintln!(
            "[WARN] Item {} not found for player {}",
            inventory_slot,
            player.uuid(),
        );
        return None;
    }
    let new_weapon = Weapon::try_from(new_weapon.unwrap());
    if let Err(e) = new_weapon {
        eprintln!(
            "[WARN] Item {} couldn't be converted to a weapon:\n{e}",
            inventory_slot,
        );
        return None;
    }
    Some(new_weapon.unwrap())
}