use std::fmt::Display;

use crate::fight_mechanics::ApplyDamageModifier;

use super::super::protection::{Destroyable, Protection, Protectable};
use super::body_side::BodySide;

#[derive(Debug)]
pub enum BodyPartKind {
    Hand(BodySide),
    Arm(BodySide),
    Torso,
    Head,
    Leg(BodySide),
}

impl Display for BodyPartKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyPartKind::Hand(side) => write!(f, "{side} hand"),
            BodyPartKind::Arm(direction) => write!(f, "{direction} arm"),
            BodyPartKind::Head => write!(f, "head"),
            BodyPartKind::Leg(direction) => write!(f, "{direction} leg"),
            BodyPartKind::Torso => write!(f, "torso"),
        }
    }
}

pub trait GetRandomFunctionalBodyPart {
    fn get_random_functional_body_part(&self) -> BodyPartKind;
}

#[derive(Debug)]
pub struct BodyPart {
    kind: BodyPartKind,
    protection: Option<Protection>,
    is_severed: bool,
}

impl BodyPart {
    pub fn new(kind: BodyPartKind) -> Self {
        Self {
            kind,
            protection: None,
            is_severed: false,
        }
    }

    pub fn kind(&self) -> &BodyPartKind {
        &self.kind
    }

    pub fn is_severed(&self) -> bool {
        self.is_severed
    }

    pub fn sever(&mut self) {
        self.is_severed = true;
    }
}

impl Protectable for BodyPart {
    fn is_protected(&self) -> bool {
        self.protection.is_some() && !self.protection.as_ref().unwrap().is_destroyed()
    }

    fn protected_by(&self) -> Option<&Protection> {
        self.protection.as_ref()
    }

    fn protected_by_mut(&mut self) -> Option<&mut Protection> {
        self.protection.as_mut()
    }

    fn attach_protection(&mut self, protection: Protection) {
        self.protection = Some(protection);
    }
}

impl ApplyDamageModifier for BodyPart {
    fn apply_damage_modifier(&self, base: u8) -> u8 {
        if self.is_protected() {
            self.protected_by().unwrap().apply_damage_modifier(base)
        } else {
            base
        }
    }
}
