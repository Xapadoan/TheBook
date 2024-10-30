use shared::{
    inventory::{Inventory, Item, MutableItems},
    shop::Shop,
};
use sqlx::MySqlPool;

use super::{ShopManagerError, ShopManagerErrorKind};

use crate::repository::sql_repository::models::NewWeaponModel;
use crate::repository::sql_repository::models::NewProtectionModel;

pub struct ShopManager<'a> {
    db_pool: &'a MySqlPool
}

impl<'a> ShopManager<'a> {
    pub fn new(db_pool: &'a MySqlPool) -> Self {
        Self { db_pool }
    }

    pub async fn read_shop(&self) -> Result<Shop, ShopManagerError> {
        let all_weapons = sqlx::query_as!(
            NewWeaponModel,
            "SELECT * FROM new_weapons;",
        )
            .fetch_all(self.db_pool)
            .await;
        if let Err(_) = all_weapons {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ReadError,
                "Read all new_weapons failed".to_string(),
            ));
        }

        let all_protections = sqlx::query_as!(
            NewProtectionModel,
            "SELECT * FROM new_protections;",
        )
            .fetch_all(self.db_pool)
            .await;
        if let Err(_) = all_protections {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ReadError,
                "Read all new_protections failed".to_string(),
            ));
        }

        let mut shop_inventory = Inventory::new();
        for protection in all_protections.unwrap().into_iter() {
            shop_inventory.add_item(Item::Protection(protection.into()));
        }
        for weapon in all_weapons.unwrap().into_iter() {
            shop_inventory.add_item(Item::Weapon(weapon.into()));
        }

        Ok(Shop::new(shop_inventory))
    }
}
