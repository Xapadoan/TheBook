use shared::{inventory::Item, unique_entity::UniqueEntity};
use uuid::Uuid;

pub struct CreatePlayerInventorySchema {
    pub uuid: String,
    pub player_uuid: String,
    pub gold: u32,
}
pub struct CreatePlayerInventorySlotSchema {
    pub uuid: String,
    pub inventory_uuid: String,
    pub weapon_uuid: Option<String>,
    pub protection_uuid: Option<String>,
}
impl CreatePlayerInventorySlotSchema {
    pub fn from_item(inventory_uuid: &Uuid, item: &Item) -> Self {
        let (weapon_uuid, protection_uuid) = match item {
            Item::Weapon(weapon) => (Some(weapon.uuid().to_string()), None),
            Item::Protection(protection) => (None, Some(protection.uuid().to_string())),
        };
        Self {
            uuid: Uuid::new_v4().to_string(),
            inventory_uuid: inventory_uuid.to_string(),
            weapon_uuid,
            protection_uuid,
        }
    }
}

pub struct UpdatePlayerInventorySchema {
    pub gold: u32,
    pub items_to_create: Vec<CreatePlayerInventorySlotSchema>,
}
