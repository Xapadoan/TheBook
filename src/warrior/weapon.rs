use std::fmt::Display;

use crate::modifiers::Modifier;
use crate::dice::{RollDamage, Dice};
use crate::warrior::stats::{Stat, StatModifier};
use crate::equipment::{HasRupture, RuptureTestResult};

pub trait MayHaveWeapon {
    fn weapon(&self) -> Option<&Weapon>;
}

pub trait MayHaveMutableWeapon {
    fn weapon_mut(&mut self) -> Option<&mut Weapon>;
}

pub trait TakeWeapon {
    fn take_weapon(&mut self) -> Option<Weapon>;
}

pub trait GiveWeapon {
    fn give_weapon(&mut self, weapon: Weapon);
}

#[derive(Debug)]
pub enum WeaponKind {
    Sword,
    GreatSword,
    Axe,
    BattleAxe,
    Hammer,
    WarHammer,
}

#[derive(Debug)]
pub struct Weapon {
    kind: WeaponKind,
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
    rupture: Option<u8>,
}

impl Weapon {
    pub fn new(kind: WeaponKind) -> Self {
        match kind {
            WeaponKind::Sword => Self {
                kind,
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-1),
                rupture: Some(4),
            },
            WeaponKind::Axe => Self {
                kind,
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-2),
                rupture: Some(3),
            },
            WeaponKind::BattleAxe => Self {
                kind,
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
                rupture: Some(3),
            },
            WeaponKind::GreatSword => Self {
                kind,
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
                rupture: Some(4),
            },
            WeaponKind::Hammer => Self {
                kind,
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-2),
                rupture: Some(4),
            },
            WeaponKind::WarHammer => Self {
                kind,
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
                rupture: Some(4),
            },
        }
    }

    pub fn is_sharp(&self) -> bool {
        match self.kind {
            WeaponKind::Axe => true,
            WeaponKind::BattleAxe => true,
            WeaponKind::GreatSword => true,
            WeaponKind::Hammer => false,
            WeaponKind::WarHammer => false,
            WeaponKind::Sword => true,
        }
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            WeaponKind::Axe => write!(f, "shitty axe"),
            WeaponKind::BattleAxe => write!(f, "coarse battle axe"),
            WeaponKind::GreatSword => write!(f, "basic great sword"),
            WeaponKind::Hammer => write!(f, "shitty hammer"),
            WeaponKind::Sword => write!(f, "basic sword"),
            WeaponKind::WarHammer => write!(f, "coarse war hammer"),
        }
    }
}

impl RollDamage for Weapon {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl StatModifier for Weapon {
    fn modify_stat(&self, base: Stat) -> Stat {
        match base {
            Stat::Attack(attack) => Stat::Attack(self.attack_modifier.apply(attack)),
            Stat::Parry(parry) => Stat::Parry(self.parry_modifier.apply(parry))
        }
    }
}

impl HasRupture for Weapon {
    fn damage_rupture(&mut self, damage: u8) {
        if self.rupture.is_none() {
            return;
        }
        let mut rup = self.rupture.unwrap();
        if damage < rup {
            rup -= damage;
        } else {
            rup = 0;
        }
        self.rupture = Some(rup);
    }

    fn is_destroyed(&self) -> bool {
        match self.rupture {
            Some(rup) => !rup > 0,
            None => false,
        }
    }

    fn rupture_test(&self) -> crate::equipment::RuptureTestResult {
        match self.rupture {
            Some(rup) => if Dice::D6.roll() > rup {
                RuptureTestResult::Success
            } else {
                RuptureTestResult::Fail
            },
            None => RuptureTestResult::Success,
        }
    }
}