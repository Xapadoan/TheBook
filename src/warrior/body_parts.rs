use std::fmt::Display;

use rand::Rng;

use crate::fight_mechanics::ApplyDamageModifier;
use crate::dice::Dice;

use super::protection::{Destroyable, Protection};

#[derive(Debug)]
pub enum BodySide {
    Left,
    Right,
}

impl BodySide {
    pub fn random() -> Self {
        match Dice::D6.roll() {
            1..=3 => Self::Right,
            4..=6 => Self::Left,
            other => panic!("D6 roll resulted in {other}")
        }
    }
}

impl Display for BodySide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodySide::Left => write!(f, "left"),
            BodySide::Right => write!(f, "right"),
        }
    }
}


#[derive(Debug)]
pub enum BodyPartKind {
    Arm(BodySide),
    Torso,
    Head,
    Leg(BodySide),
}

impl Display for BodyPartKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

pub trait GetRandomProtectedBodyPart {
    fn get_random_protected_body_part(&self) -> Option<BodyPartKind>;
}

pub trait Protectable {
    fn is_protected(&self) -> bool;
    fn protected_by(&self) -> Option<&Protection>;
    fn protected_by_mut(&mut self) -> Option<&mut Protection>;
    fn attach_protection(&mut self, protection: Protection);
}

pub trait WearProtection {
    fn can_wear_protection(&self, protection: &Protection, body_part: BodyPartKind) -> bool;
    fn wear_protection(&mut self, protection: Protection, body_part: BodyPartKind);
}

#[derive(Debug)]
pub struct BodyPart {
    kind: BodyPartKind,
    protection: Option<Protection>,
    is_destroyed: bool,
}

impl BodyPart {
    pub fn new(kind: BodyPartKind) -> Self {
        Self {
            kind,
            protection: None,
            is_destroyed: false,
        }
    }

    pub fn kind(&self) -> &BodyPartKind {
        &self.kind
    }
}

impl Destroyable for BodyPart {
    fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    fn destroy(&mut self) {
        self.is_destroyed = true;
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

#[derive(Debug)]
pub struct Body {
    head: BodyPart,
    torso: BodyPart,
    left_arm: BodyPart,
    right_arm: BodyPart,
    left_leg: BodyPart,
    right_leg: BodyPart,
}

impl Body {
    pub fn new() -> Self {
        Self {
            head: BodyPart::new(BodyPartKind::Head),
            torso: BodyPart::new(BodyPartKind::Torso),
            left_arm: BodyPart::new(BodyPartKind::Arm(BodySide::Left)),
            right_arm: BodyPart::new(BodyPartKind::Arm(BodySide::Right)),
            left_leg: BodyPart::new(BodyPartKind::Arm(BodySide::Left)),
            right_leg: BodyPart::new(BodyPartKind::Leg(BodySide::Right)),
        }
    }

    pub fn body_part_mut(&mut self, body_part: &BodyPartKind) -> &mut BodyPart {
        match body_part {
            BodyPartKind::Arm(direction) => {
                match direction {
                    BodySide::Left => &mut self.left_arm,
                    BodySide::Right => &mut self.right_arm,
                }
            },
            BodyPartKind::Head => &mut self.head,
            BodyPartKind::Leg(direction) => {
                match direction {
                    BodySide::Left => &mut self.left_leg,
                    BodySide::Right => &mut self.right_leg,
                }
            },
            BodyPartKind::Torso => &mut self.torso,
        }
    }

    pub fn body_part(&self, body_part: &BodyPartKind) -> &BodyPart {
        match body_part {
            BodyPartKind::Arm(direction) => {
                match direction {
                    BodySide::Left => &self.left_arm,
                    BodySide::Right => &self.right_arm,
                }
            },
            BodyPartKind::Head => &self.head,
            BodyPartKind::Leg(direction) => {
                match direction {
                    BodySide::Left => &self.left_leg,
                    BodySide::Right => &self.right_leg,
                }
            },
            BodyPartKind::Torso => &self.torso,
        }
    }
}

impl ApplyDamageModifier for Body {
    fn apply_damage_modifier(&self, mut base: u8) -> u8 {
        base = self.head.apply_damage_modifier(base);
        base = self.torso.apply_damage_modifier(base);
        base = self.left_arm.apply_damage_modifier(base);
        base = self.right_arm.apply_damage_modifier(base);
        base = self.left_leg.apply_damage_modifier(base);
        base = self.right_leg.apply_damage_modifier(base);
        return base;
    }
}

impl WearProtection for Body {
    fn can_wear_protection(&self, protection: &Protection, body_part: BodyPartKind) -> bool {
        let is_already_protected = match body_part {
            BodyPartKind::Arm(ref direction) => {
                match direction {
                    BodySide::Left => self.left_arm.is_protected(),
                    BodySide::Right => self.right_arm.is_protected(),
                }
            },
            BodyPartKind::Head => self.head.is_protected(),
            BodyPartKind::Leg(ref direction) => {
                match direction {
                    BodySide::Left => self.left_leg.is_protected(),
                    BodySide::Right => self.right_leg.is_protected(),
                }
            },
            BodyPartKind::Torso => self.torso.is_protected()
        };

        if is_already_protected {
            return false;
        }

        protection.can_be_equipped_on(body_part)
    }

    fn wear_protection(&mut self, protection: Protection, body_part: BodyPartKind) {
        match body_part {
            BodyPartKind::Arm(direction) => {
                match direction {
                    BodySide::Left => self.left_arm.attach_protection(protection),
                    BodySide::Right => self.right_arm.attach_protection(protection),
                }
            },
            BodyPartKind::Head => self.head.attach_protection(protection),
            BodyPartKind::Leg(direction) => {
                match direction {
                    BodySide::Left => self.left_leg.attach_protection(protection),
                    BodySide::Right => self.right_leg.attach_protection(protection),
                }
            },
            BodyPartKind::Torso => self.torso.attach_protection(protection)
        }
    }
}

impl GetRandomFunctionalBodyPart for Body {
    fn get_random_functional_body_part(&self) -> BodyPartKind {
        let mut functional_body_parts: Vec<BodyPartKind> = Vec::new();
        if !self.head.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Head);
        }
        if !self.left_arm.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Arm(BodySide::Left));
        }
        if !self.right_arm.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Arm(BodySide::Right));
        }
        if !self.torso.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Torso);
        }
        if !self.left_leg.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Leg(BodySide::Left));
        }
        if !self.right_leg.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Leg(BodySide::Right));
        }

        if functional_body_parts.len() < 1 {
            panic!("Called get_random_functional_body_part on a dead body");
        }
        let random_index = rand::thread_rng().gen_range(0..functional_body_parts.len() - 1);
        functional_body_parts.swap_remove(random_index)
    }
}

impl GetRandomProtectedBodyPart for Body {
    fn get_random_protected_body_part(&self) -> Option<BodyPartKind> {
        let mut armored_body_parts: Vec<BodyPartKind> = Vec::new();
        if !self.head.is_destroyed() {
            armored_body_parts.push(BodyPartKind::Head);
        }
        if !self.left_arm.is_destroyed() {
            armored_body_parts.push(BodyPartKind::Arm(BodySide::Left));
        }
        if !self.right_arm.is_destroyed() {
            armored_body_parts.push(BodyPartKind::Arm(BodySide::Right));
        }
        if !self.torso.is_destroyed() {
            armored_body_parts.push(BodyPartKind::Torso);
        }
        if !self.left_leg.is_destroyed() {
            armored_body_parts.push(BodyPartKind::Leg(BodySide::Left));
        }
        if !self.right_leg.is_destroyed() {
            armored_body_parts.push(BodyPartKind::Leg(BodySide::Right));
        }

        if armored_body_parts.len() < 1 {
            return None
        }
        let random_index = rand::thread_rng().gen_range(0..armored_body_parts.len() - 1);
        Some(armored_body_parts.swap_remove(random_index))
    }
}
