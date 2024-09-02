use serde::{Deserialize, Serialize};

use crate::assault::common_traits::ReduceDamages;
use crate::equipment::protection::{OptionalMutableProtection, Protection};
use crate::stats::{Stat, StatModifier};

use super::body_part::{
    BodyPart,
    BodyPartKind,
    BodySide,
    FingerName,
    OptionalBodyPart,
    OptionalMutableBodyPart,
    PROTECTABLE_BODY_PARTS,
};
use super::injury::Injuries;

pub trait HasBody {
    fn body(&self) -> &Body;
}

pub trait HasMutableBody: HasBody {
    fn body_mut(&mut self) -> &mut Body;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    left_eye: Option<BodyPart>,
    right_eye: Option<BodyPart>,
    head: Option<BodyPart>,
    torso: Option<BodyPart>,
    left_hand: Option<BodyPart>,
    right_hand: Option<BodyPart>,
    left_arm: Option<BodyPart>,
    right_arm: Option<BodyPart>,
    left_foot: Option<BodyPart>,
    right_foot: Option<BodyPart>,
    left_knee: Option<BodyPart>,
    right_knee: Option<BodyPart>,
    left_leg: Option<BodyPart>,
    right_leg: Option<BodyPart>,
    genitals: Option<BodyPart>,
    left_thumb: Option<BodyPart>,
    right_thumb: Option<BodyPart>,
    left_pointer_finger: Option<BodyPart>,
    right_pointer_finger: Option<BodyPart>,
    left_middle_finger: Option<BodyPart>,
    right_middle_finger: Option<BodyPart>,
    left_ring_finger: Option<BodyPart>,
    right_ring_finger: Option<BodyPart>,
    left_pinky_finger: Option<BodyPart>,
    right_pinky_finger: Option<BodyPart>,
}

impl Body {
    pub fn new() -> Self {
        Self {
            left_eye: Some(BodyPart::new(BodyPartKind::Eye(BodySide::Left))),
            right_eye: Some(BodyPart::new(BodyPartKind::Eye(BodySide::Right))),
            head: Some(BodyPart::new(BodyPartKind::Head)),
            torso: Some(BodyPart::new(BodyPartKind::Torso)),
            left_hand: Some(BodyPart::new(BodyPartKind::Hand(BodySide::Left))),
            right_hand: Some(BodyPart::new(BodyPartKind::Hand(BodySide::Right))),
            left_arm: Some(BodyPart::new(BodyPartKind::Arm(BodySide::Left))),
            right_arm: Some(BodyPart::new(BodyPartKind::Arm(BodySide::Right))),
            left_foot: Some(BodyPart::new(BodyPartKind::Foot(BodySide::Left))),
            right_foot: Some(BodyPart::new(BodyPartKind::Foot(BodySide::Right))),
            left_knee: Some(BodyPart::new(BodyPartKind::Knee(BodySide::Left))),
            right_knee: Some(BodyPart::new(BodyPartKind::Knee(BodySide::Right))),
            left_leg: Some(BodyPart::new(BodyPartKind::Arm(BodySide::Left))),
            right_leg: Some(BodyPart::new(BodyPartKind::Leg(BodySide::Right))),
            genitals: Some(BodyPart::new(BodyPartKind::Genitals)),
            left_thumb: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Left, FingerName::Thumb))),
            right_thumb: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Right, FingerName::Thumb))),
            left_pointer_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Left, FingerName::PointerFinger))),
            right_pointer_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Right, FingerName::PointerFinger))),
            left_middle_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Left, FingerName::MiddleFinger))),
            right_middle_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Right, FingerName::MiddleFinger))),
            left_ring_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Left, FingerName::RingFinger))),
            right_ring_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Right, FingerName::RingFinger))),
            left_pinky_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Left, FingerName::PinkyFinger))),
            right_pinky_finger: Some(BodyPart::new(BodyPartKind::Finger(BodySide::Right, FingerName::PinkyFinger))),
        }
    }
}

impl OptionalMutableBodyPart for Body {
    fn body_part_mut(&mut self, body_part: &BodyPartKind) -> &mut Option<BodyPart> {
        match body_part {
            BodyPartKind::Genitals => &mut self.genitals,
            BodyPartKind::Eye(side) => match side {
                BodySide::Left => &mut self.left_eye,
                BodySide::Right => &mut self.right_eye,
            }
            BodyPartKind::Hand(side) => match side {
                BodySide::Left => &mut self.left_hand,
                BodySide::Right => &mut self.right_hand,
            }
            BodyPartKind::Arm(side) => match side {
                BodySide::Left => &mut self.left_arm,
                BodySide::Right => &mut self.right_arm,
            }
            BodyPartKind::Head => &mut self.head,
            BodyPartKind::Foot(side) => match side {
                BodySide::Left => &mut self.left_foot,
                BodySide::Right => &mut self.right_foot,
            },
            BodyPartKind::Knee(side) => match side {
                BodySide::Left => &mut self.left_knee,
                BodySide::Right => &mut self.right_knee,
            }
            BodyPartKind::Leg(side) => match side {
                BodySide::Left => &mut self.left_leg,
                BodySide::Right => &mut self.right_leg,
            }
            BodyPartKind::Torso => &mut self.torso,
            BodyPartKind::Finger(side, name) => match side {
                BodySide::Left => match name {
                    FingerName::Thumb => &mut self.left_thumb,
                    FingerName::PointerFinger => &mut self.left_pointer_finger,
                    FingerName::MiddleFinger => &mut self.left_middle_finger,
                    FingerName::RingFinger => &mut self.left_ring_finger,
                    FingerName::PinkyFinger => &mut self.left_pinky_finger
                },
                BodySide::Right => match name {
                    FingerName::Thumb => &mut self.right_thumb,
                    FingerName::PointerFinger => &mut self.right_pointer_finger,
                    FingerName::MiddleFinger => &mut self.right_middle_finger,
                    FingerName::RingFinger => &mut self.right_ring_finger,
                    FingerName::PinkyFinger => &mut self.right_pinky_finger
                }
            }
        }
    }
}

impl OptionalBodyPart for Body {
    fn body_part(&self, body_part: &BodyPartKind) -> &Option<BodyPart> {
        match body_part {
            BodyPartKind::Genitals => &self.genitals,
            BodyPartKind::Eye(side) => match side {
                BodySide::Left => &self.left_eye,
                BodySide::Right => &self.right_eye,
            }
            BodyPartKind::Hand(side) => match side {
                BodySide::Left => &self.left_hand,
                BodySide::Right => &self.right_hand,
            }
            BodyPartKind::Arm(side) => match side {
                BodySide::Left => &self.left_arm,
                BodySide::Right => &self.right_arm,
            }
            BodyPartKind::Head => &self.head,
            BodyPartKind::Foot(side) => match side {
                BodySide::Left => &self.left_foot,
                BodySide::Right => &self.right_foot,
            }
            BodyPartKind::Knee(side) => match side {
                BodySide::Left => &self.left_knee,
                BodySide::Right => &self.right_knee,
            }
            BodyPartKind::Leg(side) => match side {
                BodySide::Left => &self.left_leg,
                BodySide::Right => &self.right_leg,
            }
            BodyPartKind::Torso => &self.torso,
            BodyPartKind::Finger(side, name) => match side {
                BodySide::Left => match name {
                    FingerName::Thumb => &self.left_thumb,
                    FingerName::PointerFinger => &self.left_pointer_finger,
                    FingerName::MiddleFinger => &self.left_middle_finger,
                    FingerName::RingFinger => &self.left_ring_finger,
                    FingerName::PinkyFinger => &self.left_pinky_finger
                },
                BodySide::Right => match name {
                    FingerName::Thumb => &self.right_thumb,
                    FingerName::PointerFinger => &self.right_pointer_finger,
                    FingerName::MiddleFinger => &self.right_middle_finger,
                    FingerName::RingFinger => &self.right_ring_finger,
                    FingerName::PinkyFinger => &self.right_pinky_finger
                }
            }
        }
    }
}

impl ReduceDamages for Body {
    fn reduce_damages(&self, damages: u8) -> u8 {
        let mut final_damages = damages;
        for body_part_kind in PROTECTABLE_BODY_PARTS {
            if let Some(body_part) = self.body_part(&body_part_kind) {
                if let Some(protection) = body_part.protection() {
                    final_damages = protection.reduce_damages(final_damages);
                }
            }
        }
        final_damages
    }
}

impl StatModifier for Body {
    fn modify_stat(&self, base: Stat) -> Stat {
        let mut stat = base;
        for injury in self.injuries() {
            stat = injury.modify_stat(stat);
        }
        stat
    }
}
