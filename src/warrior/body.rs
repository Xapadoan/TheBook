pub mod body_part;
pub mod body_side;
pub mod injury;

use injury::MayBeInjured;
use rand::Rng;

use body_part::{BodyPart, BodyPartKind, RandomFunctionalBodyPart};
use body_side::BodySide;
use crate::fight_mechanics::ApplyDamageModifier;
use crate::modifiers::Modifier;
use super::protection::{Protectable, Protection, RandomProtectedBodyPart, WearProtection};
use super::stats::{Stat, StatModifier};

#[derive(Debug)]
pub struct Body {
    head: BodyPart,
    torso: BodyPart,
    left_hand: BodyPart,
    right_hand: BodyPart,
    left_arm: BodyPart,
    right_arm: BodyPart,
    left_foot: BodyPart,
    right_foot: BodyPart,
    left_leg: BodyPart,
    right_leg: BodyPart,
}

impl Body {
    pub fn new() -> Self {
        Self {
            head: BodyPart::new(BodyPartKind::Head),
            torso: BodyPart::new(BodyPartKind::Torso),
            left_hand: BodyPart::new(BodyPartKind::Hand(BodySide::Left)),
            right_hand: BodyPart::new(BodyPartKind::Hand(BodySide::Right)),
            left_arm: BodyPart::new(BodyPartKind::Arm(BodySide::Left)),
            right_arm: BodyPart::new(BodyPartKind::Arm(BodySide::Right)),
            left_foot: BodyPart::new(BodyPartKind::Foot(BodySide::Left)),
            right_foot: BodyPart::new(BodyPartKind::Foot(BodySide::Right)),
            left_leg: BodyPart::new(BodyPartKind::Arm(BodySide::Left)),
            right_leg: BodyPart::new(BodyPartKind::Leg(BodySide::Right)),
        }
    }

    pub fn body_part_mut(&mut self, body_part: &BodyPartKind) -> &mut BodyPart {
        match body_part {
            BodyPartKind::Hand(side) => {
                match side {
                    BodySide::Left => &mut self.left_hand,
                    BodySide::Right => &mut self.right_hand,
                }
            },
            BodyPartKind::Arm(side) => {
                match side {
                    BodySide::Left => &mut self.left_arm,
                    BodySide::Right => &mut self.right_arm,
                }
            },
            BodyPartKind::Head => &mut self.head,
            BodyPartKind::Foot(side) => {
                match side {
                    BodySide::Left => &mut self.left_foot,
                    BodySide::Right => &mut self.right_foot,
                }
            },
            BodyPartKind::Leg(side) => {
                match side {
                    BodySide::Left => &mut self.left_leg,
                    BodySide::Right => &mut self.right_leg,
                }
            },
            BodyPartKind::Torso => &mut self.torso,
        }
    }

    pub fn body_part(&self, body_part: &BodyPartKind) -> &BodyPart {
        match body_part {
            BodyPartKind::Hand(side) => {
                match side {
                    BodySide::Left => &self.left_hand,
                    BodySide::Right => &self.right_hand,
                }
            },
            BodyPartKind::Arm(side) => {
                match side {
                    BodySide::Left => &self.left_arm,
                    BodySide::Right => &self.right_arm,
                }
            },
            BodyPartKind::Head => &self.head,
            BodyPartKind::Foot(side) => {
                match side {
                    BodySide::Left => &self.left_foot,
                    BodySide::Right => &self.right_foot,
                }
            },
            BodyPartKind::Leg(side) => {
                match side {
                    BodySide::Left => &self.left_leg,
                    BodySide::Right => &self.right_leg,
                }
            },
            BodyPartKind::Torso => &self.torso,
        }
    }

    pub fn random_protected_body_part_fallback_functional(&self) -> BodyPartKind {
        let protected_body_part = self.random_protected_body_part();
        if protected_body_part.is_some() {
            protected_body_part.unwrap()
        } else {
            self.random_functional_body_part()
        }
    }
}

impl ApplyDamageModifier for Body {
    fn apply_damage_modifier(&self, mut base: u8) -> u8 {
        base = self.head.apply_damage_modifier(base);
        base = self.torso.apply_damage_modifier(base);
        base = self.left_hand.apply_damage_modifier(base);
        base = self.right_hand.apply_damage_modifier(base);
        base = self.left_arm.apply_damage_modifier(base);
        base = self.right_arm.apply_damage_modifier(base);
        base = self.left_foot.apply_damage_modifier(base);
        base = self.right_foot.apply_damage_modifier(base);
        base = self.left_leg.apply_damage_modifier(base);
        base = self.right_leg.apply_damage_modifier(base);
        return base;
    }
}

impl WearProtection for Body {
    fn can_wear_protection(&self, protection: &Protection, body_part: BodyPartKind) -> bool {
        let is_already_protected = match body_part {
            BodyPartKind::Hand(ref side) => {
                match side {
                    BodySide::Left => self.left_hand.is_protected(),
                    BodySide::Right => self.right_hand.is_protected(),
                }
            },
            BodyPartKind::Arm(ref side) => {
                match side {
                    BodySide::Left => self.left_arm.is_protected(),
                    BodySide::Right => self.right_arm.is_protected(),
                }
            },
            BodyPartKind::Head => self.head.is_protected(),
            BodyPartKind::Foot(ref side) => {
                match side {
                    BodySide::Left => self.left_foot.is_protected(),
                    BodySide::Right => self.right_foot.is_protected(),
                }
            },
            BodyPartKind::Leg(ref side) => {
                match side {
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
            BodyPartKind::Hand(side) => {
                match side {
                    BodySide::Left => self.left_hand.attach_protection(protection),
                    BodySide::Right => self.right_hand.attach_protection(protection),
                }
            },
            BodyPartKind::Arm(side) => {
                match side {
                    BodySide::Left => self.left_arm.attach_protection(protection),
                    BodySide::Right => self.right_arm.attach_protection(protection),
                }
            },
            BodyPartKind::Head => self.head.attach_protection(protection),
            BodyPartKind::Foot(side) => {
                match side {
                    BodySide::Left => self.left_foot.attach_protection(protection),
                    BodySide::Right => self.right_foot.attach_protection(protection),
                }
            },
            BodyPartKind::Leg(side) => {
                match side {
                    BodySide::Left => self.left_leg.attach_protection(protection),
                    BodySide::Right => self.right_leg.attach_protection(protection),
                }
            },
            BodyPartKind::Torso => self.torso.attach_protection(protection)
        }
    }
}

impl RandomFunctionalBodyPart for Body {
    fn random_functional_body_part(&self) -> BodyPartKind {
        let mut functional_body_parts: Vec<BodyPartKind> = Vec::new();
        if !self.head.is_severed() {
            functional_body_parts.push(BodyPartKind::Head);
        }
        if !self.left_hand.is_severed() {
            functional_body_parts.push(BodyPartKind::Hand(BodySide::Left));
        }
        if !self.right_hand.is_severed() {
            functional_body_parts.push(BodyPartKind::Hand(BodySide::Right));
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
        if !self.left_foot.is_severed() {
            functional_body_parts.push(BodyPartKind::Foot(BodySide::Left));
        }
        if !self.right_foot.is_severed() {
            functional_body_parts.push(BodyPartKind::Foot(BodySide::Right));
        }
        if !self.left_leg.is_severed() {
            functional_body_parts.push(BodyPartKind::Leg(BodySide::Left));
        }
        if !self.right_leg.is_severed() {
            functional_body_parts.push(BodyPartKind::Leg(BodySide::Right));
        }

        if functional_body_parts.len() < 2 {
            panic!("Called get_random_functional_body_part on a dead body");
        }

        let random_index = rand::thread_rng().gen_range(0..functional_body_parts.len() - 1);
        functional_body_parts.swap_remove(random_index)
    }
}

impl RandomProtectedBodyPart for Body {
    fn random_protected_body_part(&self) -> Option<BodyPartKind> {
        let mut armored_body_parts: Vec<BodyPartKind> = Vec::new();
        if !self.head.is_severed() && self.head.is_protected() {
            armored_body_parts.push(BodyPartKind::Head);
        }
        if !self.left_hand.is_severed() && self.left_hand.is_protected() {
            armored_body_parts.push(BodyPartKind::Hand(BodySide::Left));
        }
        if !self.right_hand.is_severed() && self.right_hand.is_protected() {
            armored_body_parts.push(BodyPartKind::Hand(BodySide::Right));
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
        if !self.left_foot.is_severed() && self.left_foot.is_protected() {
            armored_body_parts.push(BodyPartKind::Foot(BodySide::Left));
        }
        if !self.right_foot.is_severed() && self.right_foot.is_protected() {
            armored_body_parts.push(BodyPartKind::Foot(BodySide::Right));
        }
        if !self.left_leg.is_severed() && self.left_leg.is_protected() {
            armored_body_parts.push(BodyPartKind::Leg(BodySide::Left));
        }
        if !self.right_leg.is_severed() && self.right_leg.is_protected() {
            armored_body_parts.push(BodyPartKind::Leg(BodySide::Right));
        }

        if armored_body_parts.len() < 1 {
            return None;
        } else if armored_body_parts.len() == 1 {
            return Some(armored_body_parts.swap_remove(0));
        }

        let random_index = rand::thread_rng().gen_range(0..armored_body_parts.len() - 1);
        Some(armored_body_parts.swap_remove(random_index))
    }
}

impl StatModifier for Body {
    fn modify_stat(&self, base: Stat) -> Stat {
        let mut stat = base;
        if self.head.is_injured() {
            stat = if self.head.injuries().len() > 1 {
                match stat {
                    Stat::Attack(attack) => Stat::Attack(Modifier::new(-5).apply(attack)),
                    Stat::Parry(parry) => Stat::Parry(Modifier::new(-8).apply(parry)),
                }
            } else {
                self.head.modify_stat(stat)
            }
        }
        if self.left_arm.is_injured() {
            stat = self.left_arm.modify_stat(stat);
        }
        if self.left_foot.is_injured() {
            stat = self.left_foot.modify_stat(stat);
        }
        if self.left_hand.is_injured() {
            stat = self.left_hand.modify_stat(stat);
        }
        if self.left_leg.is_injured() || self.right_leg.is_injured() {
            stat = if !self.left_leg.is_injured() {
                self.right_leg.modify_stat(stat)
            } else if !self.right_leg.is_injured() {
                self.left_leg.modify_stat(stat)
            } else {
                match stat {
                    Stat::Attack(attack) => Stat::Attack(Modifier::new(-8).apply(attack)),
                    Stat::Parry(parry) => Stat::Parry(Modifier::new(-8).apply(parry)),
                }
            }
        }
        if self.right_arm.is_injured() {
            stat = self.right_arm.modify_stat(stat);
        }
        if self.right_foot.is_injured() {
            stat = self.right_foot.modify_stat(stat);
        }
        if self.right_hand.is_injured() {
            stat = self.right_hand.modify_stat(stat);
        }
        if self.torso.is_injured() {
            stat = self.torso.modify_stat(stat);
        }
        stat
    }
}
