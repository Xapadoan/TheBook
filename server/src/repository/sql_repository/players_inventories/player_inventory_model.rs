pub struct PlayerInventoryModel {
    pub uuid: String,
    pub player_uuid: String,
    pub gold: u32,
}

pub struct PlayerInventorySlotModel {
    uuid: String,
    inventory_uuid: String,
    weapon_uuid: Option<String>,
    protection_uuid: Option<String>,
}

#[derive(Debug)]
pub struct PlayerInventoryJoinedSlotModel {
    pub gold: u32,
    pub weapon_uuid: Option<String>,
    pub protection_uuid: Option<String>,
}

pub struct PlayerInventoryFinalModel {
    player_inventory_model: PlayerInventoryModel,
    slots: Vec<PlayerInventorySlotModel>,
}
