use serde::{Deserialize, Serialize};
use shared::equipment::weapon::{Weapon, WeaponKind};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WeaponKindColumnData(pub WeaponKind);
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

impl WeaponKindColumnData {
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
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
impl From<NewWeaponModel> for Weapon {
    fn from(value: NewWeaponModel) -> Self {
        Self::new(
            Uuid::new_v4(),
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
    pub uuid: String,
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
impl TryFrom<WeaponModel> for Weapon {
    type Error = uuid::Error;
    fn try_from(value: WeaponModel) -> Result<Self, Self::Error> {
        let weapon = Self::new(
            Uuid::parse_str(&value.uuid)?,
            value.name,
            value.kind.0,
            value.is_sharp == 1,
            value.is_two_handed == 1,
            value.additional_damages,
            value.attack_stat_modifier,
            value.parry_stat_modifier,
            value.courage_stat_modifier,
            value.rupture,
        );
        Ok(weapon)
    }
}