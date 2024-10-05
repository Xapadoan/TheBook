use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::equipment::protection::{CanWearProtection, OptionalMutableProtection, Protection, ProtectionKind};
use crate::random::Random;
use crate::stats::{StatKind, StatModifier};

pub trait OptionalBodyPart {
    fn body_part(&self, body_part_kind: &BodyPartKind) -> &Option<BodyPart>;
}

pub trait OptionalMutableBodyPart: OptionalBodyPart {
    fn body_part_mut(&mut self, body_part_kind: &BodyPartKind) -> &mut Option<BodyPart>;
    fn remove_part(&mut self, body_part_kind: &BodyPartKind, severed_parts: &mut Vec<BodyPart>) {
        if let Some(part) = self.body_part_mut(body_part_kind).take() {
            severed_parts.push(part);
        }
        match body_part_kind {
            BodyPartKind::Leg(side) => self.remove_part(
                &BodyPartKind::Knee(side.clone()),
                severed_parts,
            ),
            BodyPartKind::Knee(side) => self.remove_part(
                &BodyPartKind::Foot(side.clone()),
                severed_parts,
            ),
            BodyPartKind::Arm(side) => self.remove_part(
                &BodyPartKind::Hand(side.clone()),
                severed_parts,
            ),
            BodyPartKind::Hand(side) => {
                self.remove_part(
                    &BodyPartKind::Finger(side.clone(), FingerName::Thumb),
                    severed_parts,
                );
                self.remove_part(
                    &BodyPartKind::Finger(side.clone(), FingerName::PointerFinger),
                    severed_parts,
                );
                self.remove_part(
                    &BodyPartKind::Finger(side.clone(), FingerName::MiddleFinger),
                    severed_parts,
                );
                self.remove_part(
                    &BodyPartKind::Finger(side.clone(), FingerName::RingFinger),
                    severed_parts,
                );
                self.remove_part(
                    &BodyPartKind::Finger(side.clone(), FingerName::PinkyFinger),
                    severed_parts,
                );
            },
            _ => {},
        }
    }
    fn break_part(&mut self, body_part_kind: &BodyPartKind) {
        if let Some(part) = self.body_part_mut(body_part_kind) {
            part.set_is_broken(true)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BodyPart {
    kind: BodyPartKind,
    protection: Option<Protection>,
    is_broken: bool,
}

impl BodyPart {
    pub fn kind(&self) -> &BodyPartKind {
        &self.kind
    }

    pub fn is_broken(&self) -> bool {
        self.is_broken
    }

    pub fn new(kind: BodyPartKind) -> Self {
        Self {
            kind,
            protection: None,
            is_broken: false,
        }
    }

    pub fn set_is_broken(&mut self, is_broken: bool) {
        self.is_broken = is_broken;
    }
}

impl OptionalMutableProtection for BodyPart {
    fn protection(&self) -> &Option<Protection> {
        &self.protection
    }

    fn protection_mut(&mut self) -> &mut Option<Protection> {
        &mut self.protection
    }

    fn replace_protection(&mut self, protection: Protection) -> Option<Protection> {
        match self.kind() {
            &BodyPartKind::Arm(_) => match protection.kind() {
                ProtectionKind::Armlets => self.protection_mut().replace(protection),
                _ => Some(protection),
            },
            &BodyPartKind::Foot(_) => match protection.kind() {
                ProtectionKind::Boots => self.protection_mut().replace(protection),
                _ => Some(protection),
            },
            &BodyPartKind::Hand(_) => match protection.kind() {
                ProtectionKind::Gloves => self.protection_mut().replace(protection),
                _ => Some(protection),
            },
            &BodyPartKind::Head => match protection.kind() {
                ProtectionKind::Helm => self.protection_mut().replace(protection),
                _ => Some(protection),
            },
            &BodyPartKind::Leg(_) => match protection.kind() {
                ProtectionKind::Greaves => self.protection_mut().replace(protection),
                _ => Some(protection),
            },
            &BodyPartKind::Torso => match protection.kind() {
                ProtectionKind::Breastplate |
                ProtectionKind::ChainMail |
                ProtectionKind::Gambeson => self.protection_mut().replace(protection),
                _ => Some(protection)
            }
            _ => Some(protection)
        }
    }
}

impl CanWearProtection for BodyPart {
    fn can_wear_protection(&self, protection: &Protection) -> bool {
        match protection.kind() {
            ProtectionKind::Armlets => match self.kind() {
                BodyPartKind::Arm(_) => true,
                _ => false,
            },
            ProtectionKind::Boots => match self.kind() {
                BodyPartKind::Foot(_) => true,
                _ => false,
            },
            ProtectionKind::Breastplate |
            ProtectionKind::ChainMail |
            ProtectionKind::Gambeson => match self.kind() {
                BodyPartKind::Torso => true,
                _ => false,
            },
            ProtectionKind::Gloves => match  self.kind() {
                BodyPartKind::Hand(_) => true,
                _ => false,
            },
            ProtectionKind::Greaves => match self.kind() {
                BodyPartKind::Leg(_) => true,
                _ => false,
            },
            ProtectionKind::Helm => match self.kind() {
                BodyPartKind::Head => true,
                _ => false,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FingerName {
    Thumb,
    PointerFinger,
    MiddleFinger,
    RingFinger,
    PinkyFinger,
}

impl PartialEq for FingerName {
    fn eq(&self, other: &Self) -> bool {
        match self {
            FingerName::Thumb => match other {
                FingerName::Thumb => true,
                _ => false,
            },
            FingerName::PointerFinger => match other {
                FingerName::PointerFinger => true,
                _ => false,
            },
            FingerName::MiddleFinger => match other {
                FingerName::MiddleFinger => true,
                _ => false,
            },
            FingerName::RingFinger => match other {
                FingerName::RingFinger => true,
                _ => false,
            },
            FingerName::PinkyFinger => match other {
                FingerName::PinkyFinger => true,
                _ => false,
            },
        }
    }
}

impl Random for FingerName {
    fn random() -> FingerName {
        let random_index = rand::thread_rng().gen_range(0..ALL_FINGERS.len());
        ALL_FINGERS[random_index].clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodySide {
    Left,
    Right,
}

impl BodySide {
    pub fn other(&self) -> BodySide {
        match self {
            BodySide::Left => BodySide::Right,
            BodySide::Right => BodySide::Left,
        }
    }
}

impl PartialEq for BodySide {
    fn eq(&self, other: &Self) -> bool {
        match self {
            BodySide::Left => match other {
                BodySide::Left => true,
                _ => false,
            },
            BodySide::Right => match other {
                BodySide::Right => true,
                _ => false,
            }
        }
    }
}

impl Random for BodySide {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..2) {
            0 => BodySide::Left,
            1 => BodySide::Right,
            other => panic!("Random in range 0..2 resulted in {other}"),
        }
    }
}

impl StatModifier for BodyPart {
    fn value(&self, stat: &StatKind) -> i8 {
        match &self.protection {
            Some(protection) => protection.value(stat),
            None => 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BodyPartKind {
    Eye(BodySide),
    Finger(BodySide, FingerName),
    Hand(BodySide),
    Arm(BodySide),
    Torso,
    Head,
    Foot(BodySide),
    Knee(BodySide),
    Leg(BodySide),
    Genitals,
}

pub const PROTECTABLE_BODY_PARTS: [BodyPartKind; 10] = [
    BodyPartKind::Head,
    BodyPartKind::Torso,
    BodyPartKind::Arm(BodySide::Left),
    BodyPartKind::Arm(BodySide::Right),
    BodyPartKind::Hand(BodySide::Left),
    BodyPartKind::Hand(BodySide::Right),
    BodyPartKind::Leg(BodySide::Left),
    BodyPartKind::Leg(BodySide::Right),
    BodyPartKind::Foot(BodySide::Left),
    BodyPartKind::Foot(BodySide::Right),
];

pub const ALL_FINGERS: [FingerName; 5] = [
    FingerName::Thumb,
    FingerName::PointerFinger,
    FingerName::MiddleFinger,
    FingerName::RingFinger,
    FingerName::PinkyFinger,
];
