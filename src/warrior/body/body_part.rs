use std::fmt::Display;

use crate::fight_mechanics::ApplyDamageModifier;

use super::super::stats::{Stat, StatModifier};
use super::super::protection::{Destroyable, Protection, Protectable};
use super::body_side::BodySide;
use super::injury::{Injury, MayBeInjured};

#[derive(Debug)]
pub enum BodyPartKind {
    Eye(BodySide),
    Hand(BodySide),
    Arm(BodySide),
    Torso,
    Head,
    Foot(BodySide),
    Leg(BodySide),
}

impl Display for BodyPartKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyPartKind::Hand(side) => write!(f, "{side} hand"),
            BodyPartKind::Arm(side) => write!(f, "{side} arm"),
            BodyPartKind::Head => write!(f, "head"),
            BodyPartKind::Foot(side) => write!(f, "{side} foot"),
            BodyPartKind::Leg(side) => write!(f, "{side} leg"),
            BodyPartKind::Torso => write!(f, "torso"),
            BodyPartKind::Eye(side) => write!(f, "{side} eye"),
        }
    }
}

pub trait RandomFunctionalBodyPart {
    fn random_functional_body_part(&self) -> BodyPartKind;
}

#[derive(Debug)]
pub struct BodyPart {
    kind: BodyPartKind,
    protection: Option<Protection>,
    is_severed: bool,
    injuries: Vec<Injury>,
}

impl BodyPart {
    pub fn new(kind: BodyPartKind) -> Self {
        Self {
            kind,
            protection: None,
            is_severed: false,
            injuries: Vec::new(),
        }
    }

    pub fn kind(&self) -> &BodyPartKind {
        &self.kind
    }

    pub fn is_severed(&self) -> bool {
        self.is_severed
    }

    pub fn sever(&mut self) -> Option<Protection> {
        self.is_severed = true;
        self.detach_protection()
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

    fn detach_protection(&mut self) -> Option<Protection> {
        self.protection.take()
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

impl MayBeInjured for BodyPart {
    fn is_injured(&self) -> bool {
        self.injuries.len() > 0        
    }

    fn injuries(&self) -> &Vec<Injury> {
        &self.injuries
    }

    fn add_injury(&mut self, injury: Injury) {
        self.injuries.push(injury);
    }
}

impl StatModifier for BodyPart {
    fn modify_stat(&self, base: Stat) -> Stat {
        let mut stat = base;
        for injury in self.injuries() {
            stat = injury.modify_stat(stat)
        }
        stat
    }
}
