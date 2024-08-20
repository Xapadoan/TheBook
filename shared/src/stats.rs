use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Stat {
    Attack(u8),
    Parry(u8),
}

impl Stat {
    pub fn value(&self) -> u8 {
        match self {
            Self::Attack(value) => *value,
            Self::Parry(value) => *value,
        }
    }

    pub fn modify(&self, modifier: i8) -> Stat {
        let new_value = match self.value().checked_add_signed(modifier) {
            Some(v) => v,
            None => if modifier > 0 {
                u8::MAX
            } else {
                0
            }
        };
        match self {
            Self::Attack(_) => Self::Attack(new_value),
            Self::Parry(_) => Self::Parry(new_value),
        }
    }
}

pub trait StatModifier {
    fn modify_stat(&self, base: Stat) -> Stat;
}

pub trait Stats {
    fn stats(&self) -> &StatsManager;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsManager {
    nat_attack: Stat,
    nat_parry: Stat,
}

impl StatsManager {
    pub fn new() -> Self {
        Self {
            nat_attack: Stat::Attack(8),
            nat_parry: Stat::Parry(10),
        }
    }

    pub fn attack(&self) -> &Stat {
        &self.nat_attack
    }

    pub fn parry(&self) -> &Stat {
        &self.nat_parry
    }
}
