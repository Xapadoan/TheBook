use shared::{
    equipment::protection::OptionalMutableProtection,
    warrior::body::body_part::BodyPart,
};

use super::{ShowSelf, ShowSelfExtended};

impl ShowSelfExtended for BodyPart {
    fn show_self_extended(&self) -> String {
        let mut str = self.kind().show_self();
        if self.is_broken() {
            str += " (broken)";
        }
        if let Some(protection) = self.protection() {
            str += protection.show_self_extended().as_str();
        }

        str
    }
}
