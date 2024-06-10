use rand::Rng;

use crate::fight_mechanics::ApplyDamageModifier;
use crate::horizontal_direction::HorizontalDirection;

use super::protection::{Destroyable, Protection};

#[derive(Debug)]
pub enum BodyPartKind {
    Arm(HorizontalDirection),
    Torso,
    Head,
    Leg(HorizontalDirection),
}

pub trait GetRandomFunctionalBodyPart {
    fn get_random_functional_body_part(&self) -> BodyPartKind;
}

trait Protectable {
    fn is_protected(&self) -> bool;
    fn protected_by(&self) -> Option<&Protection>;
    fn attach_protection(&mut self, protection: Protection);
}

pub trait WearProtection {
    fn can_wear_protection(&self, protection: &Protection, body_part: BodyPartKind) -> bool;
    fn wear_protection(&mut self, protection: Protection, body_part: BodyPartKind);
}

#[derive(Debug)]
pub struct BodyPart {
    protection: Option<Protection>,
    is_destroyed: bool,
}

impl BodyPart {
    pub fn new() -> Self {
        Self {
            protection: None,
            is_destroyed: false,
        }
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
        self.protection.is_some()
    }

    fn protected_by(&self) -> Option<&Protection> {
        self.protection.as_ref()
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
            head: BodyPart::new(),
            torso: BodyPart::new(),
            left_arm: BodyPart::new(),
            right_arm: BodyPart::new(),
            left_leg: BodyPart::new(),
            right_leg: BodyPart::new(),
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
                    HorizontalDirection::Left => self.left_arm.is_protected(),
                    HorizontalDirection::Right => self.right_arm.is_protected(),
                }
            },
            BodyPartKind::Head => self.head.is_protected(),
            BodyPartKind::Leg(ref direction) => {
                match direction {
                    HorizontalDirection::Left => self.left_leg.is_protected(),
                    HorizontalDirection::Right => self.right_leg.is_protected(),
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
                    HorizontalDirection::Left => self.left_arm.attach_protection(protection),
                    HorizontalDirection::Right => self.right_arm.attach_protection(protection),
                }
            },
            BodyPartKind::Head => self.head.attach_protection(protection),
            BodyPartKind::Leg(direction) => {
                match direction {
                    HorizontalDirection::Left => self.left_leg.attach_protection(protection),
                    HorizontalDirection::Right => self.right_leg.attach_protection(protection),
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
            functional_body_parts.push(BodyPartKind::Arm(HorizontalDirection::Left));
        }
        if !self.right_arm.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Arm(HorizontalDirection::Right));
        }
        if !self.torso.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Torso);
        }
        if !self.left_leg.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Leg(HorizontalDirection::Left));
        }
        if !self.right_leg.is_destroyed() {
            functional_body_parts.push(BodyPartKind::Leg(HorizontalDirection::Right));
        }

        let random_index = rand::thread_rng().gen_range(0..functional_body_parts.len() - 1);
        functional_body_parts.swap_remove(random_index)
    }
}
