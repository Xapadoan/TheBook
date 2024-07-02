use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Stat {
    Attack(u8),
    Parry(u8),
}

impl Stat {
    pub fn consume(stat: Stat) -> u8 {
        match stat {
            Self::Attack(value) => value,
            Self::Parry(value) => value,
        }
    }
}

pub trait StatModifier {
    fn modify_stat(&self, base: Stat) -> Stat;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsManager {
    nat_attack: u8,
    nat_parry: u8,
}

impl StatsManager {
    pub fn new() -> Self {
        Self {
            nat_attack: 8,
            nat_parry: 10,
        }
    }

    pub fn attack_stat(&self) -> Stat {
        Stat::Attack(self.nat_attack)
    }

    pub fn parry_stat(&self) -> Stat {
        Stat::Parry(self.nat_parry)
    }
}