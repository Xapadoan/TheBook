use shared::{equipment::{protection::Protection, rupture::Rupture}, name::Name, stats::{StatKind, StatModifier}};
use uuid::Uuid;

use super::protection_model::ProtectionKindColumnData;

pub struct CreateProtectionSchema {
    pub uuid: Uuid,
    pub name: String,
    pub kind: ProtectionKindColumnData,
    pub rupture: Option<u8>,
    pub damage_reduction: u8,
    pub courage_stat_modifier: i8,
    pub dexterity_stat_modifier: i8,
}

impl From<Protection> for CreateProtectionSchema {
    fn from(value: Protection) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: value.name().to_string(),
            kind: ProtectionKindColumnData(value.kind().clone()),
            rupture: value.rupture().clone(),
            damage_reduction: value.amount(),
            courage_stat_modifier: value.value(&StatKind::Courage),
            dexterity_stat_modifier: value.value(&StatKind::Dexterity),
        }
    }
}

pub struct UpdateProtectionSchema {
    pub rupture: Option<u8>,
}
