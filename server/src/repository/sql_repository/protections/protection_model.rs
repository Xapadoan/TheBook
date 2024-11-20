use serde::{Deserialize, Serialize};
use shared::equipment::protection::{Protection, ProtectionKind};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProtectionKindColumnData(pub ProtectionKind);
impl ProtectionKindColumnData {
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
}
impl From<String> for ProtectionKindColumnData {
    fn from(value: String) -> Self {
        if value == "Armlets" {
            ProtectionKindColumnData(ProtectionKind::Armlets)
        } else if value == "Boots" {
            ProtectionKindColumnData(ProtectionKind::Boots)
        } else if value == "Breastplate" {
            ProtectionKindColumnData(ProtectionKind::Breastplate)
        } else if value == "ChainMail" {
            ProtectionKindColumnData(ProtectionKind::ChainMail)
        } else if value == "Gambeson" {
            ProtectionKindColumnData(ProtectionKind::Gambeson)
        } else if value == "Gloves" {
            ProtectionKindColumnData(ProtectionKind::Gloves)
        } else if value == "Greaves" {
            ProtectionKindColumnData(ProtectionKind::Greaves)
        } else if value == "Helm" {
            ProtectionKindColumnData(ProtectionKind::Helm)
        } else {
            panic!("Failed to create WeaponKindColumnData from String: {value}")
        }
    }
}

#[derive(Debug, Serialize, Deserialize,  FromRow)]
pub struct NewProtectionModel {
    pub id: u32,
    pub name: String,
    pub kind: ProtectionKindColumnData,
    pub rupture: Option<u8>,
    pub damage_reduction: u8,
    pub courage_stat_modifier: i8,
    pub dexterity_stat_modifier: i8,
}
impl From<NewProtectionModel> for Protection {
    fn from(protection: NewProtectionModel) -> Self {
        Self::new(
            Uuid::new_v4(),
            protection.name,
            protection.kind.0,
            protection.damage_reduction,
            protection.rupture,
            protection.dexterity_stat_modifier,
            protection.courage_stat_modifier,
        )
    }
}

#[derive(Debug, Serialize, Deserialize,  FromRow)]
pub struct ProtectionModel {
    pub uuid: String,
    pub name: String,
    pub kind: ProtectionKindColumnData,
    pub rupture: Option<u8>,
    pub damage_reduction: u8,
    pub courage_stat_modifier: i8,
    pub dexterity_stat_modifier: i8,
}
impl TryFrom<ProtectionModel> for Protection {
    type Error = uuid::Error;

    fn try_from(value: ProtectionModel) -> Result<Self, Self::Error> {
        let protection = Self::new(
            Uuid::parse_str(&value.uuid)?,
            value.name,
            value.kind.0,
            value.damage_reduction,
            value.rupture,
            value.dexterity_stat_modifier,
            value.courage_stat_modifier,
        );

        Ok(protection)
    }
}
