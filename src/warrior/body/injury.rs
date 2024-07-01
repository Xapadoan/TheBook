use crate::{modifiers::Modifier, warrior::stats::{Stat, StatModifier}};

pub trait MayBeInjured {
    fn is_injured(&self) -> bool;
    fn is_severed(&self) -> bool;
    // fn is_dislocated(&self) -> bool;
    // fn is_broken(&self) -> bool;
    fn is_gouged(&self) -> bool;
    fn injuries(&self) -> &Vec<Injury>;
    fn add_injury(&mut self, injury: Injury);
}

pub trait MayCauseInjury {
    fn injury(&self) -> Option<&Injury>;
}

pub trait TakeInjury {
    fn take_injury(&mut self) -> Option<Injury>;
}

#[derive(Debug)]
pub enum InjuryKind {
    Severed,
    Dislocated,
    Broken,
    Gouged,
}

#[derive(Debug)]
pub struct Injury {
    kind: InjuryKind,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
    // reason: String,
}

impl Injury {
    // pub fn new(kind: InjuryKind, attack: i8, parry: i8, reason: String)-> Self {
    pub fn new(kind: InjuryKind, attack: i8, parry: i8)-> Self {
        Self {
            kind,
            attack_modifier: Modifier::new(attack),
            parry_modifier: Modifier::new(parry),
            // reason,
        }
    }

    // pub fn reason(&self) -> &String {
    //     &self.reason
    // }

    pub fn kind(&self) -> &InjuryKind {
        &self.kind
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
