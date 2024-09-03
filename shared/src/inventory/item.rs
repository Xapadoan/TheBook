use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::equipment::{protection::Protection, weapon::Weapon};

use super::{error::InventoryErrorKind, InventoryError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Item {
    Weapon(Weapon),
    Protection(Protection),
}

impl TryFrom<Item> for Weapon {
    type Error = InventoryError<Item>;

    fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
            Item::Protection(_) => Err(InventoryError::new(&InventoryErrorKind::NotAWeapon, value)),
            Item::Weapon(weapon) => Ok(weapon)
        }
    }
}

pub trait MutableItems {
    fn add_item(&mut self, item: Item) -> Option<Item>;
    fn remove_item(&mut self, index: &Uuid) -> Option<Item>;
}
