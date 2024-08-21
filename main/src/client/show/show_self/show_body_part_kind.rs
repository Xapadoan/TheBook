use shared::warrior::body::body_part::{BodyPartKind, BodySide, FingerName};

use super::ShowSelf;

impl ShowSelf for BodySide {
    fn show_self(&self) -> String {
        match self {
            BodySide::Left => "left".to_string(),
            BodySide::Right => "right".to_string(),
        }
    }
}

impl ShowSelf for FingerName {
    fn show_self(&self) -> String {
        match self {
            FingerName::Thumb => "thumb".to_string(),
            FingerName::PointerFinger => "pointer finger".to_string(),
            FingerName::MiddleFinger => "middle finger".to_string(),
            FingerName::RingFinger => "ring finger".to_string(),
            FingerName::PinkyFinger => "pinky finger".to_string(),
        }
    }
}

impl ShowSelf for BodyPartKind {
    fn show_self(&self) -> String {
        match self {
            BodyPartKind::Arm(side) => format!("{} arm", side.show_self()),
            BodyPartKind::Eye(side) => format!("{} eye", side.show_self()),
            BodyPartKind::Finger(side, finger) => format!("{} {}", side.show_self(), finger.show_self()),
            BodyPartKind::Foot(side) => format!("{} foot", side.show_self()),
            BodyPartKind::Genitals => "genitals".to_string(),
            BodyPartKind::Hand(side) => format!("{} hand", side.show_self()),
            BodyPartKind::Head => "head".to_string(),
            BodyPartKind::Knee(side) => format!("{} knee", side.show_self()),
            BodyPartKind::Leg(side) => format!("{} leg", side.show_self()),
            BodyPartKind::Torso => "torso".to_string(),
        }
    }
}
