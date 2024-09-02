use shared::equipment::protection::{OptionalMutableProtection, Protection};
use shared::warrior::body::body_part::{BodyPartKind, OptionalMutableBodyPart};
use shared::warrior::body::HasMutableBody;
use shared::warrior::Warrior;

pub trait TakeProtections {
    fn take_protections(&mut self, parts: Vec<&BodyPartKind>) -> Vec<Protection>;
}

impl TakeProtections for Warrior {
    fn take_protections(&mut self, parts: Vec<&BodyPartKind>) -> Vec<Protection> {
        let mut protections = vec![];
        for body_part_kind in parts {
            if let Some(body_part) = self.body_mut().body_part_mut(body_part_kind) {
                if let Some(protection) = body_part.protection_mut().take() {
                    protections.push(protection);
                }
            }
        }
        protections
    }
}
