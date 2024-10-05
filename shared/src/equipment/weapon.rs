use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::assault::common_traits::DealDamages;
use crate::dice::Dice;
use crate::name::Name;
use crate::random::Random;
use crate::stats::{StatKind, StatModifier};

use super::rupture::Rupture;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WeaponKind {
    Sword,
    GreatSword,
    Axe,
    BattleAxe,
    Hammer,
    WarHammer,
}

impl Random for WeaponKind {
    fn random() -> Self {
        match rand::thread_rng().gen_range(1..=6) {
            1 => WeaponKind::Sword,
            2 => WeaponKind::GreatSword,
            3 => WeaponKind::Axe,
            4 => WeaponKind::BattleAxe,
            5 => WeaponKind::Hammer,
            6 => WeaponKind::WarHammer,
            other => panic!("{other} not in range"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weapon {
    name: String,
    kind: WeaponKind,
    is_sharp: bool,
    is_two_handed: bool,
    rupture: Option<u8>,
    add_dmg: u8,
    attack_mod: i8,
    parry_mod: i8,
    courage_mod: i8,
}

impl Weapon {
    pub fn new(kind: WeaponKind) -> Self {
        match kind {
            WeaponKind::Sword => Self {
                name: String::from("Shitty Sword"),
                kind,
                is_sharp: true,
                is_two_handed: false,
                add_dmg: 3,
                attack_mod: 0,
                parry_mod: -1,
                courage_mod: -1,
                rupture: Some(4),
            },
            WeaponKind::Axe => Self {
                name: String::from("Rusty Axe"),
                kind,
                is_sharp: true,
                is_two_handed: false,
                add_dmg: 3,
                attack_mod: 0,
                parry_mod: -2,
                courage_mod: 0,
                rupture: Some(3),
            },
            WeaponKind::BattleAxe => Self {
                name: String::from("Coarse Battle Axe"),
                kind,
                is_sharp: true,
                is_two_handed: true,
                add_dmg: 5,
                attack_mod: -3,
                parry_mod: -4,
                courage_mod: 0,
                rupture: Some(3),
            },
            WeaponKind::GreatSword => Self {
                name: String::from("Basic Great Sword"),
                kind,
                is_sharp: true,
                is_two_handed: true,
                add_dmg: 5,
                attack_mod: -3,
                parry_mod: -4,
                courage_mod: 0,
                rupture: Some(4),
            },
            WeaponKind::Hammer => Self {
                name: String::from("Shitty Hammer"),
                kind,
                is_sharp: false,
                is_two_handed: false,
                add_dmg: 3,
                attack_mod: 0,
                parry_mod: -2,
                courage_mod: 0,
                rupture: Some(4),
            },
            WeaponKind::WarHammer => Self {
                name: String::from("Coarse War Hammer"),
                kind,
                is_sharp: false,
                is_two_handed: true,
                add_dmg: 5,
                attack_mod: -3,
                parry_mod: -4,
                courage_mod: 0,
                rupture: Some(4),
            },
        }
    }

    pub fn is_sharp(&self) -> bool {
        self.is_sharp
    }

    pub fn is_two_handed(&self) -> bool {
        self.is_two_handed
    }

    pub fn additional_damages(&self) -> u8 {
        self.add_dmg
    }
}

pub trait OptionalMutableWeapon {
    fn weapon(&self) -> &Option<Weapon>;
    fn weapon_mut(&mut self) -> &mut Option<Weapon>;
    fn replace_weapon(&mut self, weapon: Weapon) -> Option<Weapon> {
        self.weapon_mut().replace(weapon)
    }
}

impl Rupture for Weapon {
    fn rupture(&self) -> &Option<u8> {
        &self.rupture
    }

    fn set_rupture(&mut self, rup: Option<u8>) {
        self.rupture = rup
    }
}

impl Random for Weapon {
    fn random() -> Self {
       Self::new(WeaponKind::random()) 
    }
}

impl DealDamages for Weapon {
    fn deal_damages(&self) -> u8 {
        Dice::D6.roll() + self.add_dmg
    }
}

impl StatModifier for Weapon {
    fn value(&self, stat: &StatKind) -> i8 {
        match stat {
            &StatKind::Attack => self.attack_mod,
            &StatKind::Parry => self.parry_mod,
            &StatKind::Courage => self.courage_mod,
            &StatKind::Dexterity => 0,
            &StatKind::Strength => 0,
        }
    }
}

impl Name for Weapon {
    fn name(&self) -> &str {
        &self.name
    }
}
