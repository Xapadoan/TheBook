use shared::{
    inventory::{GoldValue, HasInventory, HasMutableInventory, Item, MutableItems}, player::Player, unique_entity::UniqueEntity
};
use uuid::Uuid;

use crate::{repository::{PlayerRepository, Repository}, shop::ShopManager};

use super::PlayerAPIError;

pub fn buy_item(player: &mut Player, slot_uuid: &Uuid) -> Result<Option<Item>, PlayerAPIError> {
    let mut shop = ShopManager::read_shop()?;
    match shop.inventory_mut().remove_item(slot_uuid) {
        None => Ok(None),
        Some(item) => if player.inventory().gold() < item.gold_value() {
            Ok(None)
        } else {
            player.inventory_mut().remove_gold(item.gold_value());
            player.inventory_mut().add_item(item.clone());
            let repo = PlayerRepository::build()?;
            repo.update(player.uuid(), &player)?;
            Ok(Some(item))
        }
    }
}

pub fn sell_item(player: &mut Player, slot_uuid: &Uuid) -> Result<Option<u32>, PlayerAPIError> {
    match player.inventory_mut().remove_item(slot_uuid) {
        None => Ok(None),
        Some(item) => {
            let value = item.gold_value() * 2 / 3;
            dbg!(&value);
            player.inventory_mut().add_gold(value);
            dbg!(player.inventory().gold());
            let repo = PlayerRepository::build()?;
            repo.update(player.uuid(), &player)?;
            Ok(Some(value))
        },
    }
}
