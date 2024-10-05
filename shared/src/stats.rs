use serde::{Deserialize, Serialize};

use crate::{dice::Dice, random::Random};

#[derive(Debug, Clone)]
pub enum StatKind {
    Attack,
    Parry,
    Courage,
    Dexterity,
    Strength,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Stat {
    Attack(u8),
    Parry(u8),
    Strength(u8),
    Dexterity(u8),
    Courage(u8),
}

impl Stat {
    pub fn value(&self) -> u8 {
        match self {
            Self::Attack(value) => *value,
            Self::Parry(value) => *value,
            Self::Strength(value) => *value,
            Self::Dexterity(value) => *value,
            Self::Courage(value) => *value,
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
            Self::Strength(_) => Self::Strength(new_value),
            Self::Dexterity(_) => Self::Dexterity(new_value),
            Self::Courage(_) => Self::Courage(new_value),
        }
    }
}

pub trait StatModifier {
    fn modify_stat(&self, base: Stat) -> Stat {
        match base {
            Stat::Attack(_) => base.modify(self.value(&StatKind::Attack)),
            Stat::Parry(_) => base.modify(self.value(&StatKind::Parry)),
            Stat::Strength(_) => base.modify(self.value(&StatKind::Strength)),
            Stat::Dexterity(_) => base.modify(self.value(&StatKind::Dexterity)),
            Stat::Courage(_) => base.modify(self.value(&StatKind::Courage)),
        }
    }
    fn value(&self, stat: &StatKind) -> i8;
}

pub trait Stats {
    fn stats(&self) -> &StatsManager;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatsManager {
    nat_attack: Stat,
    nat_parry: Stat,
    nat_strength: Stat,
    nat_dexterity: Stat,
    nat_courage: Stat,
}

impl StatsManager {
    pub fn nat_stat(&self, stat: &StatKind) -> &Stat {
        match stat {
            StatKind::Attack => &self.nat_attack,
            StatKind::Parry => &self.nat_parry,
            StatKind::Courage => &self.nat_courage,
            StatKind::Dexterity => &self.nat_dexterity,
            StatKind::Strength => &self.nat_strength,
        }
    }

    pub fn stat(&self, modifiers: &[Box<&dyn StatModifier>], stat: &StatKind) -> Stat {
        let mut real = self.nat_stat(stat).clone();
        for modifier in modifiers {
            real = real.modify(modifier.value(stat));
        }
        match stat {
            StatKind::Attack => {
                let dexterity = self.stat(modifiers, &StatKind::Dexterity).value();
                if dexterity > 12 {
                    real = real.modify(dexterity as i8 - 12);
                } else if dexterity < 9 {
                    real = real.modify(-1);
                }
            },
            _ => {},
        }

        real
    }

    pub fn increment_nat_stat(&mut self, stat: &StatKind) {
        match stat {
            &StatKind::Attack => self.nat_attack = self.nat_attack.modify(1),
            &StatKind::Parry => self.nat_parry = self.nat_parry.modify(1),
            &StatKind::Courage => self.nat_courage = self.nat_courage.modify(1),
            &StatKind::Dexterity => self.nat_dexterity = self.nat_dexterity.modify(1),
            &StatKind::Strength => self.nat_strength = self.nat_strength.modify(1),
        }
    }
}

impl Random for StatsManager {
    fn random() -> Self {
        Self {
            nat_attack: Stat::Attack(8),
            nat_parry: Stat::Parry(10),
            nat_strength: Stat::Strength(Dice::D6.roll() + 7),
            nat_dexterity: Stat::Dexterity(Dice::D6.roll() + 7),
            nat_courage: Stat::Courage(Dice::D6.roll() + 7),
        }
    }
}
