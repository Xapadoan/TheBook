use std::fmt::Display;

use crate::fight_mechanics::ApplyDamageModifier;
use crate::equipment::HasRupture;

use super::super::stats::{Stat, StatModifier};
use super::super::protection::{Protection, Protectable};
use super::body_side::BodySide;
use super::injury::{Injury, InjuryKind, MayBeInjured};

pub trait MayTargetBodyPart {
    fn target_body_part(&self) -> Option<&BodyPartKind>;
}

#[derive(Debug)]
pub enum BodyPartKind {
    Eye(BodySide),
    Hand(BodySide),
    Arm(BodySide),
    Torso,
    Head,
    Foot(BodySide),
    Knee(BodySide),
    Leg(BodySide),
    Genitals,
}

impl Display for BodyPartKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyPartKind::Hand(side) => write!(f, "{side} hand"),
            BodyPartKind::Arm(side) => write!(f, "{side} arm"),
            BodyPartKind::Head => write!(f, "head"),
            BodyPartKind::Foot(side) => write!(f, "{side} foot"),
            BodyPartKind::Leg(side) => write!(f, "{side} leg"),
            BodyPartKind::Knee(side) => write!(f, "{side} knee"),
            BodyPartKind::Torso => write!(f, "torso"),
            BodyPartKind::Eye(side) => write!(f, "{side} eye"),
            BodyPartKind::Genitals => write!(f, "genitals"),
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
    injuries: Vec<Injury>,
}

impl BodyPart {
    pub fn new(kind: BodyPartKind) -> Self {
        Self {
            kind,
            protection: None,
            injuries: Vec::new(),
        }
    }

    pub fn kind(&self) -> &BodyPartKind {
        &self.kind
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

    fn is_broken(&self) -> bool {
        for injury in self.injuries() {
            match injury.kind() {
                InjuryKind::Broken => return true,
                _ => {},
            }
        }
        false
    }

    fn is_dislocated(&self) -> bool {
        for injury in self.injuries() {
            match injury.kind() {
                InjuryKind::Dislocated => return true,
                _ => {},
            }
        }
        false
    }

    fn is_gouged(&self) -> bool {
        for injury in self.injuries() {
            match injury.kind() {
                InjuryKind::Gouged => return true,
                _ => {},
            }
        }
        false
    }

    fn is_severed(&self) -> bool {
        for injury in self.injuries() {
            match injury.kind() {
                InjuryKind::Severed => return true,
                _ => {},
            }
        }
        false
    }

    fn injuries(&self) -> &Vec<Injury> {
        &self.injuries
    }

    fn add_injury(&mut self, injury: Injury) {
        match injury.kind() {
            InjuryKind::Severed => self.injuries.clear(),
            InjuryKind::Broken => self.injuries.retain(
                |injury|
                match injury.kind() {
                    InjuryKind::Broken | InjuryKind::Dislocated => false,
                    _ => true,
                }
            ),
            InjuryKind::Gouged => self.injuries.retain(
                |injury|
                match injury.kind() {
                    InjuryKind::Gouged => false,
                    _ => true,
                }
            ),
            InjuryKind::Dislocated => self.injuries.retain(
                |injury|
                match injury.kind() {
                    InjuryKind::Dislocated => false,
                    _ => true,
                }
            )
        }
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
