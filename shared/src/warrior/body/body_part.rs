use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::equipment::protection::{OptionalMutableProtection, Protection};
use crate::random::Random;

use super::injury::Injury;

pub trait OptionalBodyPart {
    fn body_part(&self, body_part_kind: &BodyPartKind) -> &Option<BodyPart>;
}

pub trait OptionalMutableBodyPart: OptionalBodyPart {
    fn body_part_mut(&mut self, body_part_kind: &BodyPartKind) -> &mut Option<BodyPart>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyPart {
    kind: BodyPartKind,
    protection: Option<Protection>,
    injury: Option<Injury>,
}

impl BodyPart {
    pub fn kind(&self) -> &BodyPartKind {
        &self.kind
    }

    pub fn injury(&self) -> &Option<Injury> {
        &self.injury
    }

    pub fn new(kind: BodyPartKind) -> Self {
        Self {
            kind,
            protection: None,
            injury: None,
        }
    }
}

impl OptionalMutableProtection for BodyPart {
    fn protection(&self) -> &Option<Protection> {
        &self.protection
    }

    fn protection_mut(&mut self) -> &mut Option<Protection> {
        &mut self.protection
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

const ALL_FINGERS: [FingerName; 5] = [
    FingerName::Thumb,
    FingerName::PointerFinger,
    FingerName::MiddleFinger,
    FingerName::RingFinger,
    FingerName::PinkyFinger,
];
