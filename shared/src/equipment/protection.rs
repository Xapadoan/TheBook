use serde::{Deserialize, Serialize};

use crate::{assault::common_traits::ReduceDamages, name::Name, stats::StatModifier};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Protection {
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

    pub fn new(kind: ProtectionKind) -> Self {
        match kind {
            ProtectionKind::Armlets => Self {
                kind,
                amount: 1,
                rupture: Some(5),
                dexterity_mod: -2,
                courage_mod: 0,
                name: "Heavy coarse metal armlet".to_string(),
            },
            ProtectionKind::Boots => Self {
                kind,
                amount: 0,
                rupture: Some(5),
                dexterity_mod: -1,
                courage_mod: 0,
                name: "Shabby leather boots".to_string(),
            },
            ProtectionKind::Breastplate => Self {
                kind,
                amount: 3,
                rupture: Some(4),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Basic leather breastplate".to_string(),
            },
            ProtectionKind::ChainMail => Self {
                kind,
                amount: 3,
                rupture: Some(4),
                dexterity_mod: -1,
                courage_mod: 0,
                name: "Rusty chain mail".to_string(),
            },
            ProtectionKind::Gambeson => Self {
                kind,
                amount: 2,
                rupture: Some(4),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Basic gambeson".to_string(),
            },
            ProtectionKind::Gloves => Self {
                kind,
                amount: 0,
                rupture: Some(5),
                dexterity_mod: 0,
                courage_mod: 0,
                name: "Leather Gloves".to_string(),
            },
            ProtectionKind::Greaves => Self {
                kind,
                amount: 1,
                rupture: Some(5),
                dexterity_mod: -2,
                courage_mod: 0,
                name: "Heavy coarse greaves".to_string(),
            },
            ProtectionKind::Helm => Self {
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
    fn attack_mod(&self) -> i8 {
        0
    }
    fn parry_mod(&self) -> i8 {
        0
    }
    fn courage_mod(&self) -> i8 {
        self.courage_mod
    }
    fn dexterity_mod(&self) -> i8 {
        self.dexterity_mod
    }
    fn strength_mod(&self) -> i8 {
        0
    }
}
