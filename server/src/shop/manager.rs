use std::{fs, path::PathBuf};

use shared::{
    equipment::{protection::{Protection, ProtectionKind}, weapon::{Weapon, WeaponKind}},
    inventory::{Inventory, Item, MutableItems},
    shop::Shop,
};
use sqlx::MySqlPool;

use super::{ShopManagerError, ShopManagerErrorKind};

use crate::repository::sql_repository::models::NewWeaponModel;
use crate::repository::sql_repository::models::NewProtectionModel;

const SHOP_INVENTORY_DIR: &'static str = "data/shop";
const SHOP_INVENTORY_NAME: &'static str = "inventory.json";

pub struct ShopManager<'a> {
    db_pool: &'a MySqlPool
}

impl<'a> ShopManager<'a> {
    pub fn new(db_pool: &'a MySqlPool) -> Self {
        Self { db_pool }
    }

    fn path() -> String {
        SHOP_INVENTORY_DIR.to_string() + "/" + SHOP_INVENTORY_NAME
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

        // let serialized_shop = fs::read_to_string(Self::path());
        // if let Err(_) = serialized_shop {
        //     return Err(ShopManagerError::new(
        //         &ShopManagerErrorKind::ReadError,
        //         format!("Read from path {} failed", Self::path()),
        //     ))
        // }
        // let serialized_shop = serialized_shop.unwrap();
        // let shop = serde_json::from_str(&serialized_shop);
        // if let Err(_) = shop {
        //     return Err(ShopManagerError::new(
        //         &ShopManagerErrorKind::ReadError,
        //         format!("Deserialization Failed:\n{}", serialized_shop),
        //     ));
        // }
        // let shop = shop.unwrap();
        Ok(Shop::new(shop_inventory))
    }

    pub fn reset_shop() -> Result<(), ShopManagerError> {
        let mut inventory = Inventory::new();

        let weapon = Weapon::from_kind(WeaponKind::Axe);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::from_kind(WeaponKind::BattleAxe);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::from_kind(WeaponKind::GreatSword);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::from_kind(WeaponKind::Hammer);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::from_kind(WeaponKind::Sword);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::from_kind(WeaponKind::WarHammer);
        inventory.add_item(Item::Weapon(weapon));

        let protection = Protection::from_kind(ProtectionKind::Armlets);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::Boots);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::Breastplate);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::ChainMail);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::Gambeson);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::Gloves);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::Greaves);
        inventory.add_item(Item::Protection(protection));
        let protection = Protection::from_kind(ProtectionKind::Helm);
        inventory.add_item(Item::Protection(protection));

        let shop = Shop::new(inventory);

        let dir_exist = PathBuf::from(SHOP_INVENTORY_DIR).as_path().try_exists();
        if let Err(e) = dir_exist {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ResetError,
                format!("Can't check dir {SHOP_INVENTORY_DIR} existence:\n{e}")
            ))
        }
        let dir_exist = dir_exist.unwrap();
        if !dir_exist {
            if let Err(e) = fs::create_dir_all(SHOP_INVENTORY_DIR) {
                return Err(ShopManagerError::new(
                    &ShopManagerErrorKind::ResetError,
                    format!("Can't create dir {SHOP_INVENTORY_DIR}:\n{e}")
                ))
            }
        }

        let serialized_shop = serde_json::to_string(&shop);
        if let Err(_) = serialized_shop {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ResetError,
                format!("Serialization Failed:\n{:?}", shop),
            ));
        }
        let serialized_shop = serialized_shop.unwrap();
        if let Err(_) = fs::write(Self::path(), &serialized_shop) {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ResetError,
                format!("Write to path {} failed", Self::path()),
            ))
        }
        Ok(())
    }
}
