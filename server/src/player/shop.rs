use shared::{
    inventory::{GoldValue, HasInventory, HasMutableInventory, Item, MutableItems},
    unique_entity::UniqueEntity,
};
use uuid::Uuid;

use crate::{repository::{PlayerRepository, Repository}, shop::ShopManager};

use super::PlayerAPIError;

pub fn buy_item(player_uuid: &Uuid, slot_uuid: &Uuid) -> Result<Option<Item>, PlayerAPIError> {
    let repo = PlayerRepository::build()?;
    let mut player = repo.get_by_uuid(player_uuid)?;
    let mut shop = ShopManager::read_shop()?;
    match shop.inventory_mut().remove_item(slot_uuid) {
        None => Ok(None),
        Some(item) => if player.inventory().gold() < item.gold_value() {
            Ok(None)
        } else {
            player.inventory_mut().remove_gold(item.gold_value());
            player.inventory_mut().add_item(item.clone());
            repo.update(player.uuid(), &player)?;
            Ok(Some(item))
        }
    }
}

pub fn sell_item(player_uuid: &Uuid, slot_uuid: &Uuid) -> Result<Option<u32>, PlayerAPIError> {
    let repo = PlayerRepository::build()?;
    let mut player = repo.get_by_uuid(player_uuid)?;
    match player.inventory_mut().remove_item(slot_uuid) {
        None => Ok(None),
        Some(item) => {
            let value = item.gold_value() * 2 / 3;
            dbg!(&value);
            player.inventory_mut().add_gold(value);
            dbg!(player.inventory().gold());
            repo.update(player.uuid(), &player)?;
            Ok(Some(value))
        },
    }
}
