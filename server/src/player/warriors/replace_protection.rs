use shared::{
    equipment::protection::{CanWearProtection, OptionalMutableProtection, Protection},
    warrior::{
        body::{body_part::{BodyPartKind, OptionalMutableBodyPart}, HasMutableBody},
        Warrior,
    },
};

pub trait ReplaceProtection: CanWearProtection {
    fn replace_protection(&mut self, body_part_kind: &BodyPartKind, protection: Protection) -> Option<Protection>;
}

impl ReplaceProtection for Warrior {
    fn replace_protection(&mut self, body_part_kind: &BodyPartKind, protection: Protection) -> Option<Protection> {
        match self.body_mut().body_part_mut(body_part_kind) {
            None => Some(protection),
            Some(body_part) => body_part.replace_protection(protection)
        }
    }
}