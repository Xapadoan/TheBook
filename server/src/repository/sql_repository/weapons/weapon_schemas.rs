use shared::{
    equipment::{
        rupture::Rupture,
        weapon::Weapon,
    },
    name::Name,
    stats::{StatKind, StatModifier},
};
use uuid::Uuid;

use super::weapon_model::WeaponKindColumnData;

pub struct CreateWeaponSchema {
    pub uuid: Uuid,
    pub name: String,
    pub kind: WeaponKindColumnData,
    pub is_sharp: i8,
    pub is_two_handed: i8,
    pub rupture: Option<u8>,
    pub additional_damages: u8,
    pub attack_stat_modifier: i8,
    pub parry_stat_modifier: i8,
    pub courage_stat_modifier: i8,
}

impl From<Weapon> for CreateWeaponSchema {
    fn from(value: Weapon) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: value.name().to_string(),
            kind: WeaponKindColumnData(value.kind().clone()),
            is_sharp: i8::from(value.is_sharp()),
            is_two_handed: i8::from(value.is_two_handed()),
            additional_damages: value.additional_damages(),
            attack_stat_modifier: value.value(&StatKind::Attack),
            parry_stat_modifier: value.value(&StatKind::Parry),
            courage_stat_modifier: value.value(&StatKind::Courage),
            rupture: value.rupture().clone(),
        }
    }
}

pub struct UpdateWeaponSchema {
    pub rupture: Option<u8>
}