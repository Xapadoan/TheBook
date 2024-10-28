use serde::{Deserialize, Serialize};
use shared::equipment::protection::{Protection, ProtectionKind};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::Model;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProtectionKindColumnData(ProtectionKind);
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
impl Model for NewProtectionModel {
    fn table_name() -> &'static str {
        "new_protections"
    }
}
impl From<NewProtectionModel> for Protection {
    fn from(protection: NewProtectionModel) -> Self {
        Self::new(
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
    pub uuid: Uuid,
    pub name: String,
    pub kind: ProtectionKindColumnData,
    pub rupture: Option<u8>,
    pub damage_reduction: u8,
    pub courage_stat_modifier: i8,
    pub dexterity_stat_modifier: i8,
}
impl Model for ProtectionModel {
    fn table_name() -> &'static str {
        "protections"
    }
}
