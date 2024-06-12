use std::fmt::Display;

use crate::equipment::{HasRupture, RuptureTestResult};
use crate::fight_mechanics::ApplyDamageModifier;
use crate::modifiers::Modifier;
use crate::dice::Dice;

use super::body::body_part::BodyPartKind;

pub trait RandomProtectedBodyPart {
    fn random_protected_body_part(&self) -> Option<BodyPartKind>;
}

pub trait Protectable {
    fn is_protected(&self) -> bool;
    fn protected_by(&self) -> Option<&Protection>;
    fn protected_by_mut(&mut self) -> Option<&mut Protection>;
    fn attach_protection(&mut self, protection: Protection);
    fn detach_protection(&mut self) -> Option<Protection>;
}

pub trait WearProtection {
    fn can_wear_protection(&self, protection: &Protection, body_part: BodyPartKind) -> bool;
    fn wear_protection(&mut self, protection: Protection, body_part: BodyPartKind);
}

#[derive(Debug)]
pub enum ProtectionKind {
    Gambeson,
    Jacket,
    Plastron,
    Greave,
    Armlet,
    ChainMail,
    Helm,
    Gauntlet,
    Boot,
}

#[derive(Debug)]
pub struct Protection {
    kind: ProtectionKind,
    dmg_modifier: Modifier,
    rupture: Option<u8>,
    display_name: String,
}

impl Protection {
    pub fn new(kind: ProtectionKind) -> Self {
        match kind {
            ProtectionKind::Boot => Self {
                kind: ProtectionKind::Boot,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                display_name: String::from("shabby leather boots")
            },
            ProtectionKind::Gauntlet => Self {
                kind: ProtectionKind::Gauntlet,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                display_name: String::from("leather gauntlet"),
            },
            ProtectionKind::Armlet => Self {
                kind: ProtectionKind::Armlet,
                dmg_modifier: Modifier::new(-1),
                rupture: Some(5),
                display_name: String::from("heavy coarse metal armlet"),
            },
            ProtectionKind::ChainMail => Self {
                kind: ProtectionKind::ChainMail,
                dmg_modifier: Modifier::new(-4),
                rupture: Some(3),
                display_name: String::from("sleeveless basic chain mail"),
            },
            ProtectionKind::Gambeson => Self {
                kind: ProtectionKind::Gambeson,
                dmg_modifier: Modifier::new(-2),
                rupture: Some(4),
                display_name: String::from("basic gambeson with sleeves"),
            },
            ProtectionKind::Greave => Self {
                kind: ProtectionKind::Armlet,
                dmg_modifier: Modifier::new(-1),
                rupture: Some(5),
                display_name: String::from("heavy coarse metal greave"),
            },
            ProtectionKind::Helm => Self {
                kind: ProtectionKind::Helm,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                display_name: String::from("Leather helmet"),
            },
            ProtectionKind::Jacket => Self {
                kind: ProtectionKind::Jacket,
                dmg_modifier: Modifier::new(-2),
                rupture: Some(5),
                display_name: String::from("reinforced canvas jacket with sleeves"),
            },
            ProtectionKind::Plastron => Self {
                kind: ProtectionKind::Armlet,
                dmg_modifier: Modifier::new(-3),
                rupture: Some(4),
                display_name: String::from("basic leather plastron"),
            },
        }
    }

    pub fn can_be_equipped_on(&self, body_part: BodyPartKind) -> bool {
        match self.kind {
            ProtectionKind::Boot => {
                match body_part {
                    BodyPartKind::Foot(_) => true,
                    _ => false,
                }
            }
            ProtectionKind::Gauntlet => {
                match body_part {
                    BodyPartKind::Hand(_) => true,
                    _ => false,
                }
            }
            ProtectionKind::Armlet => {
                match body_part {
                    BodyPartKind::Arm(_) => true,
                    _ => false,
                }
            },
            ProtectionKind::ChainMail => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
            ProtectionKind::Gambeson => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
            ProtectionKind::Greave => {
                match body_part {
                    BodyPartKind::Leg(_) => true,
                    _ => false,
                }
            },
            ProtectionKind::Helm => {
                match body_part {
                    BodyPartKind::Head => true,
                    _ => false,
                }
            },
            ProtectionKind::Jacket => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
            ProtectionKind::Plastron => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
        }
    }
}

impl Display for Protection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

impl HasRupture for Protection {
    fn damage_rupture(&mut self, damage: u8) {
        if self.rupture.is_none() {
            return;
        }
        let mut rup = self.rupture.unwrap();
        if damage < rup {
            rup -= damage;
        } else {
            rup = 0;
        }
        self.rupture = Some(rup);
    }

    fn is_destroyed(&self) -> bool {
        match self.rupture {
            Some(rup) => !rup > 0,
            None => false,
        }
    }

    fn rupture_test(&self) -> crate::equipment::RuptureTestResult {
        match self.rupture {
            Some(rup) => if Dice::D6.roll() > rup {
                RuptureTestResult::Success
            } else {
                RuptureTestResult::Fail
            },
            None => RuptureTestResult::Success,
        }
    }
}

impl ApplyDamageModifier for Protection {
    fn apply_damage_modifier(&self, base: u8) -> u8 {
        if self.is_destroyed() {
            base
        } else {
            self.dmg_modifier.apply(base)
        }
    }
}
