use std::{fs, path::PathBuf};

use shared::{
    equipment::weapon::{Weapon, WeaponKind},
    inventory::{Inventory, Item, MutableItems},
    shop::Shop,
};

use super::{ShopManagerError, ShopManagerErrorKind};

const SHOP_INVENTORY_DIR: &'static str = "data/shop";
const SHOP_INVENTORY_NAME: &'static str = "inventory.json";

pub struct ShopManager {}

impl ShopManager {
    fn path() -> String {
        SHOP_INVENTORY_DIR.to_string() + "/" + SHOP_INVENTORY_NAME
    }

    pub fn read_shop() -> Result<Shop, ShopManagerError> {
        let serialized_shop = fs::read_to_string(Self::path());
        if let Err(_) = serialized_shop {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ReadError,
                format!("Read from path {} failed", Self::path()),
            ))
        }
        let serialized_shop = serialized_shop.unwrap();
        let shop = serde_json::from_str(&serialized_shop);
        if let Err(_) = shop {
            return Err(ShopManagerError::new(
                &ShopManagerErrorKind::ReadError,
                format!("Deserialization Failed:\n{}", serialized_shop),
            ));
        }
        let shop = shop.unwrap();
        Ok(shop)
    }

    pub fn reset_shop() -> Result<(), ShopManagerError> {
        let mut inventory = Inventory::new();
        let weapon = Weapon::new(WeaponKind::Axe);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::new(WeaponKind::BattleAxe);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::new(WeaponKind::GreatSword);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::new(WeaponKind::Hammer);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::new(WeaponKind::Sword);
        inventory.add_item(Item::Weapon(weapon));
        let weapon = Weapon::new(WeaponKind::WarHammer);
        inventory.add_item(Item::Weapon(weapon));
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
