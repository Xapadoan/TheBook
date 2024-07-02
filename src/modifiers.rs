use serde::{Deserialize, Serialize};

pub trait ApplyDamageModifier {
    fn apply_damage_modifier(&self, base: u8) -> u8;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modifier {
    value: i8,
}

impl Modifier {
    pub fn new(value: i8) -> Modifier {
        Modifier { value }
    }

    pub fn apply(&self, base: u8) -> u8 {
        match base.checked_add_signed(self.value) {
            Some(value) => value,
            None => 0,
        }
    }
}

