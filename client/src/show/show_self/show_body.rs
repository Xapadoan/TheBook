use shared::warrior::body::{
    body_part::{BodyPartKind, BodySide, FingerName, OptionalBodyPart},
    Body,
};

use super::ShowSelfExtended;

impl ShowSelfExtended for Body {
    fn show_self_extended(&self) -> String {
        let mut str = String::new();
        str += format!(
            "Head: {}\nEyes: {}\t{}\nTorso: {}",
            self.body_part(&BodyPartKind::Head).show_self_extended(),
            self.body_part(&BodyPartKind::Eye(BodySide::Right)).show_self_extended(),
            self.body_part(&BodyPartKind::Eye(BodySide::Left)).show_self_extended(),
            self.body_part(&BodyPartKind::Torso).show_self_extended(),
        ).as_str();
        str += format!(
            "\nArms: {}\t{}\nHands: {}\t{}",
            self.body_part(&BodyPartKind::Arm(BodySide::Right)).show_self_extended(),
            self.body_part(&BodyPartKind::Arm(BodySide::Left)).show_self_extended(),
            self.body_part(&BodyPartKind::Hand(BodySide::Right)).show_self_extended(),
            self.body_part(&BodyPartKind::Hand(BodySide::Left)).show_self_extended(),
        ).as_str();
        str += format!(
            "\nThumbs: {}\t{}",
            self.body_part(&BodyPartKind::Finger(BodySide::Right, FingerName::Thumb)).show_self_extended(),
            self.body_part(&BodyPartKind::Finger(BodySide::Left, FingerName::Thumb)).show_self_extended(),
        ).as_str();
        str += format!(
            "\nPointer Fingers: {}\t{}",
            self.body_part(&BodyPartKind::Finger(BodySide::Right, FingerName::PointerFinger)).show_self_extended(),
            self.body_part(&BodyPartKind::Finger(BodySide::Left, FingerName::PointerFinger)).show_self_extended(),
        ).as_str();
        str += format!(
            "\nMiddle Fingers: {}\t{}",
            self.body_part(&BodyPartKind::Finger(BodySide::Right, FingerName::MiddleFinger)).show_self_extended(),
            self.body_part(&BodyPartKind::Finger(BodySide::Left, FingerName::MiddleFinger)).show_self_extended(),
        ).as_str();
        str += format!(
            "\nRing Fingers: {}\t{}",
            self.body_part(&BodyPartKind::Finger(BodySide::Right, FingerName::RingFinger)).show_self_extended(),
            self.body_part(&BodyPartKind::Finger(BodySide::Left, FingerName::RingFinger)).show_self_extended(),
        ).as_str();
        str += format!(
            "\nPinkies: {}\t{}",
            self.body_part(&BodyPartKind::Finger(BodySide::Right, FingerName::PinkyFinger)).show_self_extended(),
            self.body_part(&BodyPartKind::Finger(BodySide::Left, FingerName::PinkyFinger)).show_self_extended(),
        ).as_str();
        str += format!(
            "\nLegs: {}\t{}\nKnees:{}\t{}\nFeet: {}\t{}",
            self.body_part(&BodyPartKind::Leg(BodySide::Right)).show_self_extended(),
            self.body_part(&BodyPartKind::Leg(BodySide::Left)).show_self_extended(),
            self.body_part(&BodyPartKind::Knee(BodySide::Right)).show_self_extended(),
            self.body_part(&BodyPartKind::Knee(BodySide::Left)).show_self_extended(),
            self.body_part(&BodyPartKind::Foot(BodySide::Right)).show_self_extended(),
            self.body_part(&BodyPartKind::Foot(BodySide::Left)).show_self_extended(),
        ).as_str();

        str
    }
}