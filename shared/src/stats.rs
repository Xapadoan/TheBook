use serde::{Deserialize, Serialize};

use crate::{dice::Dice, random::Random};

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
    fn attack_mod(&self) -> i8;
    fn parry_mod(&self) -> i8;
    fn strength_mod(&self) -> i8;
    fn dexterity_mod(&self) -> i8;
    fn courage_mod(&self) -> i8;
    fn modify_stat(&self, base: Stat) -> Stat {
        match base {
            Stat::Attack(_) => base.modify(self.attack_mod()),
            Stat::Parry(_) => base.modify(self.parry_mod()),
            Stat::Strength(_) => base.modify(self.strength_mod()),
            Stat::Dexterity(_) => base.modify(self.dexterity_mod()),
            Stat::Courage(_) => base.modify(self.courage_mod()),
        }
    }
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
    pub fn attack(&self, modifiers: &[Box<&dyn StatModifier>]) -> Stat {
        let mut attack = self.nat_attack.clone();
        let mut dexterity = self.nat_dexterity.clone();
        for modifier in modifiers {
            attack = modifier.modify_stat(attack);
            dexterity = modifier.modify_stat(dexterity);
        }
        if dexterity.value() < 9 {
            attack.modify(-1)
        } else if dexterity.value() > 12 {
            attack.modify(dexterity.value() as i8 - 12)
        }else {
            attack
        }
    }

    pub fn parry(&self, modifiers: &[Box<&dyn StatModifier>]) -> Stat {
        let mut parry = self.nat_parry.clone();
        for modifier in modifiers {
            parry = modifier.modify_stat(parry);
        }
        parry
    }

    pub fn strength(&self, modifiers: &[Box<&dyn StatModifier>]) -> Stat {
        let mut str = self.nat_strength.clone();
        for modifier in modifiers {
            str = modifier.modify_stat(str);
        }
        str
    }

    pub fn dexterity(&self, modifiers: &[Box<&dyn StatModifier>]) -> Stat {
        let mut dex = self.nat_dexterity.clone();
        for modifier in modifiers {
            dex = modifier.modify_stat(dex);
        }
        dex
    }

    pub fn courage(&self, modifiers: &[Box<&dyn StatModifier>]) -> Stat {
        let mut cou = self.nat_courage.clone();
        for modifier in modifiers {
            cou = modifier.modify_stat(cou);
        }
        cou
    }

    pub fn increment_nat_stat(&mut self, stat: &Stat) {
        match stat {
            &Stat::Attack(_) => self.nat_attack = self.nat_attack.modify(1),
            &Stat::Parry(_) => self.nat_parry = self.nat_parry.modify(1),
            &Stat::Courage(_) => self.nat_courage = self.nat_courage.modify(1),
            &Stat::Dexterity(_) => self.nat_dexterity = self.nat_dexterity.modify(1),
            &Stat::Strength(_) => self.nat_strength = self.nat_strength.modify(1),
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
