use serde::{Deserialize, Serialize};
use shared::equipment::weapon::{Weapon, WeaponKind};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::Model;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WeaponKindColumnData(WeaponKind);
impl From<String> for WeaponKindColumnData {
    fn from(value: String) -> Self {
        if value == "Sword" {
            WeaponKindColumnData(WeaponKind::Sword)
        } else if value == "GreatSword" {
            WeaponKindColumnData(WeaponKind::GreatSword)
        } else if value == "Axe" {
            WeaponKindColumnData(WeaponKind::Axe)
        } else if value == "BattleAxe" {
            WeaponKindColumnData(WeaponKind::BattleAxe)
        } else if value == "Hammer" {
            WeaponKindColumnData(WeaponKind::Hammer)
        } else if value == "WarHammer" {
            WeaponKindColumnData(WeaponKind::WarHammer)
        } else {
            panic!("Failed to create WeaponKindColumnData from String: {value}")
        }
    }
}

#[derive(Debug, Serialize, Deserialize,  FromRow)]
pub struct NewWeaponModel {
    pub id: u32,
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
impl Model for NewWeaponModel {
    fn table_name() -> &'static str {
        "new_weapons"
    }
}
impl From<NewWeaponModel> for Weapon {
    fn from(value: NewWeaponModel) -> Self {
        Self::new(
            value.name,
            value.kind.0,
            value.is_sharp == 1,
            value.is_two_handed == 1,
            value.additional_damages,
            value.attack_stat_modifier,
            value.parry_stat_modifier,
            value.courage_stat_modifier,
            value.rupture,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WeaponModel {
    pub uuid: Uuid,
    pub name: String,
    pub kind: WeaponKind,
    pub is_sharp: i8,
    pub is_two_handed: i8,
    pub rupture: Option<u8>,
    pub additional_damages: u8,
    pub attack_stat_modifier: i8,
    pub parry_stat_modifier: i8,
    pub courage_stat_modifier: i8,
}
impl Model for WeaponModel {
    fn table_name() -> &'static str {
        "weapons"
    }
}
