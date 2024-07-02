use std::fmt::Display;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::equipment::{HasRupture, RuptureTestResult, RUPTURE_MAX};
use crate::gen_random::GenRandom;
use crate::modifiers::{ApplyDamageModifier, Modifier};
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
    // fn detach_protection(&mut self) -> Option<Protection>;
}

pub trait WearProtection {
    fn can_wear_protection(&self, protection: &Protection, body_part: BodyPartKind) -> bool;
    fn wear_protection(&mut self, protection: Protection, body_part: BodyPartKind);
}

#[derive(Debug, Serialize, Deserialize)]
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

impl GenRandom for ProtectionKind {
    fn gen_random() -> Self {
        match rand::thread_rng().gen_range(1..=9) {
            1 => ProtectionKind::Armlet,
            2 => ProtectionKind::Boot,
            3 => ProtectionKind::ChainMail,
            4 => ProtectionKind::Gambeson,
            5 => ProtectionKind::Gauntlet,
            6 => ProtectionKind::Greave,
            7 => ProtectionKind::Helm,
            8 => ProtectionKind::Jacket,
            9 => ProtectionKind::Plastron,
            other => panic!("{other} is out of range")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
                kind,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                display_name: String::from("shabby leather boots")
            },
            ProtectionKind::Gauntlet => Self {
                kind,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                display_name: String::from("leather gauntlet"),
            },
            ProtectionKind::Armlet => Self {
                kind,
                dmg_modifier: Modifier::new(-1),
                rupture: Some(5),
                display_name: String::from("heavy coarse metal armlet"),
            },
            ProtectionKind::ChainMail => Self {
                kind,
                dmg_modifier: Modifier::new(-4),
                rupture: Some(3),
                display_name: String::from("sleeveless basic chain mail"),
            },
            ProtectionKind::Gambeson => Self {
                kind,
                dmg_modifier: Modifier::new(-2),
                rupture: Some(4),
                display_name: String::from("basic gambeson with sleeves"),
            },
            ProtectionKind::Greave => Self {
                kind,
                dmg_modifier: Modifier::new(-1),
                rupture: Some(5),
                display_name: String::from("heavy coarse metal greave"),
            },
            ProtectionKind::Helm => Self {
                kind,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                display_name: String::from("Leather helmet"),
            },
            ProtectionKind::Jacket => Self {
                kind,
                dmg_modifier: Modifier::new(-2),
                rupture: Some(5),
                display_name: String::from("reinforced canvas jacket with sleeves"),
            },
            ProtectionKind::Plastron => Self {
                kind,
                dmg_modifier: Modifier::new(-3),
                rupture: Some(4),
                display_name: String::from("basic leather plastron"),
            },
        }
    }

    pub fn kind<'a>(&'a self) -> &'a ProtectionKind {
        &self.kind
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
        rup = match rup.checked_add(damage) {
            Some(result) => result,
            None => RUPTURE_MAX + 1,
        };
        self.rupture = Some(rup);
    }

    fn is_destroyed(&self) -> bool {
        match self.rupture {
            Some(rup) => rup > RUPTURE_MAX,
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

impl GenRandom for Protection {
    fn gen_random() -> Self {
        Self::new(ProtectionKind::gen_random())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protection_damage_rupture() {
        let mut plastron = Protection::new(ProtectionKind::Plastron);
        assert_eq!(plastron.rupture, Some(4));
        assert!(!plastron.is_destroyed());
        plastron.damage_rupture(1);
        assert_eq!(plastron.rupture, Some(5));
        assert!(!plastron.is_destroyed());
        plastron.damage_rupture(1);
        assert_eq!(plastron.rupture, Some(6));
        assert!(plastron.is_destroyed());

        let mut plastron = Protection::new(ProtectionKind::Plastron);
        assert!(!plastron.is_destroyed());
        plastron.damage_rupture(u8::MAX);
        assert!(plastron.is_destroyed());
    }

    #[test]
    fn should_apply_damage_modifier_unless_destroyed() {
        let mut gambeson = Protection::new(ProtectionKind::Gambeson);

        let raw_damage = 9;
        let reduced_damage = gambeson.apply_damage_modifier(raw_damage);
        assert!(
            raw_damage > reduced_damage,
            "Protection didn't reduce damage (raw: {}, reduced: {})", raw_damage, reduced_damage
        );

        gambeson.damage_rupture(gambeson.rupture.unwrap());
        let reduced_damage = gambeson.apply_damage_modifier(raw_damage);
        assert_eq!(raw_damage, reduced_damage);
    }
}
