pub mod body_part;
pub mod body_side;

use rand::Rng;

use body_part::{BodyPart, BodyPartKind, GetRandomFunctionalBodyPart};
use body_side::BodySide;
use crate::fight_mechanics::ApplyDamageModifier;
use super::protection::{Protection, WearProtection, GetRandomProtectedBodyPart, Protectable};

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
        if !self.head.is_severed() {
            functional_body_parts.push(BodyPartKind::Head);
        }
        if !self.left_arm.is_severed() {
            functional_body_parts.push(BodyPartKind::Arm(BodySide::Left));
        }
        if !self.right_arm.is_severed() {
            functional_body_parts.push(BodyPartKind::Arm(BodySide::Right));
        }
        if !self.torso.is_severed() {
            functional_body_parts.push(BodyPartKind::Torso);
        }
        if !self.left_leg.is_severed() {
            functional_body_parts.push(BodyPartKind::Leg(BodySide::Left));
        }
        if !self.right_leg.is_severed() {
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
        if !self.head.is_severed() && self.head.is_protected() {
            armored_body_parts.push(BodyPartKind::Head);
        }
        if !self.left_arm.is_severed() && self.left_arm.is_protected() {
            armored_body_parts.push(BodyPartKind::Arm(BodySide::Left));
        }
        if !self.right_arm.is_severed() && self.right_arm.is_protected() {
            armored_body_parts.push(BodyPartKind::Arm(BodySide::Right));
        }
        if !self.torso.is_severed() && self.torso.is_protected() {
            armored_body_parts.push(BodyPartKind::Torso);
        }
        if !self.left_leg.is_severed() && self.left_leg.is_protected() {
            armored_body_parts.push(BodyPartKind::Leg(BodySide::Left));
        }
        if !self.right_leg.is_severed() && self.right_leg.is_protected() {
            armored_body_parts.push(BodyPartKind::Leg(BodySide::Right));
        }

        if armored_body_parts.len() < 1 {
            return None
        }
        let random_index = rand::thread_rng().gen_range(0..armored_body_parts.len() - 1);
        Some(armored_body_parts.swap_remove(random_index))
    }
}
