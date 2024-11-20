use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{assault::common_traits::ReduceDamages, name::Name, stats::{StatKind, StatModifier}, unique_entity::UniqueEntity};

use super::rupture::Rupture;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProtectionKind {
    Armlets,
    Boots,
    Breastplate,
    ChainMail,
    Gambeson,
    Gloves,
    Greaves,
    Helm,
}
impl ProtectionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Armlets => "Armlets",
            Self::Boots => "Boots",
            Self::Breastplate => "Breastplate",
            Self::ChainMail => "ChainMail",
            Self::Gambeson => "Gambeson",
            Self::Gloves => "Gloves",
            Self::Greaves => "Greaves",
            Self::Helm => "Helm",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Protection {
    uuid: Uuid,
    kind: ProtectionKind,
    amount: u8,
    rupture: Option<u8>,
    dexterity_mod: i8,
    courage_mod: i8,
    name: String,
}

impl Protection {
    pub fn kind(&self) -> &ProtectionKind {
        &self.kind
    }

    pub fn amount(&self) -> u8 {
        self.amount
    }
    pub fn new(
        uuid: Uuid,
        name: String,
        kind: ProtectionKind,
        amount: u8,
        rupture: Option<u8>,
        dexterity_mod: i8,
        courage_mod: i8,
    ) -> Self {
        Self {
            uuid,
            name,
            kind,
            amount,
            rupture,
            dexterity_mod,
            courage_mod,
        }
    }

    pub fn from_kind(kind: ProtectionKind) -> Self {
        match kind {
            ProtectionKind::Armlets => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 1,
                rupture: Some(5),
                dexterity_mod: -2,
                courage_mod: 0,
                name: "Heavy coarse metal armlet".to_string(),
            },
            ProtectionKind::Boots => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 0,
                rupture: Some(5),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Shabby leather boots".to_string(),
            },
            ProtectionKind::Breastplate => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 3,
                rupture: Some(4),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Basic leather breastplate".to_string(),
            },
            ProtectionKind::ChainMail => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 3,
                rupture: Some(4),
                dexterity_mod: -1,
                courage_mod: 0,
                name: "Rusty chain mail".to_string(),
            },
            ProtectionKind::Gambeson => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 2,
                rupture: Some(4),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Basic gambeson".to_string(),
            },
            ProtectionKind::Gloves => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 0,
                rupture: Some(5),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Leather Gloves".to_string(),
            },
            ProtectionKind::Greaves => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 1,
                rupture: Some(5),
                dexterity_mod: -2,
                courage_mod: 0,
                name: "Heavy coarse greaves".to_string(),
            },
            ProtectionKind::Helm => Self {
                uuid: Uuid::new_v4(),
                kind,
                amount: 0,
                rupture: Some(5),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Shabby leather helmet".to_string(),
            }
        }
    }
}

pub trait OptionalMutableProtection {
    fn protection(&self) -> &Option<Protection>;
    fn protection_mut(&mut self) -> &mut Option<Protection>;
    fn replace_protection(&mut self, protection: Protection) -> Option<Protection>;
}

pub trait CanWearProtection {
    fn can_wear_protection(&self, protection: &Protection) -> bool;
}

impl ReduceDamages for Protection {
    fn reduce_damages(&self, damages: u8) -> u8 {
        if damages > self.amount {
            damages - self.amount
        } else {
            0
        }
    }
}

impl Rupture for Protection {
    fn rupture(&self) -> &Option<u8> {
        &self.rupture
    }

    fn set_rupture(&mut self, rup: Option<u8>) {
        self.rupture = rup;
    }
}

impl Name for Protection {
    fn name(&self) -> &str {
        &self.name
    }
}

impl StatModifier for Protection {
    fn value(&self, stat: &StatKind) -> i8 {
        match stat {
            &StatKind::Dexterity => self.dexterity_mod,
            _ => 0,
        }
    }
}
impl UniqueEntity for Protection {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
