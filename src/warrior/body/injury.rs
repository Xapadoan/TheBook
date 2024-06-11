use crate::{modifiers::Modifier, warrior::stats::{Stat, StatModifier}};

pub trait MayBeInjured {
    fn is_injured(&self) -> bool;
    fn injuries(&self) -> &Vec<Injury>;
    fn add_injury(&mut self, injury: Injury);
}

#[derive(Debug)]
pub struct Injury {
    attack_modifier: Modifier,
    parry_modifier: Modifier,
    reason: String,
}

impl Injury {
    pub fn new(attack: i8, parry: i8, reason: String)-> Self {
        Self {
            attack_modifier: Modifier::new(attack),
            parry_modifier: Modifier::new(parry),
            reason,
        }
    }

    pub fn reason(&self) -> &String {
        &self.reason
    }
}

impl StatModifier for Injury {
    fn modify_stat(&self, base: Stat) -> Stat {
        match base {
            Stat::Attack(attack) => Stat::Attack(self.attack_modifier.apply(attack)),
            Stat::Parry(parry) => Stat::Parry(self.parry_modifier.apply(parry)),
        }
    }
}
