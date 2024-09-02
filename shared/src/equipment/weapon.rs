use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{assault::common_traits::DealDamages, dice::Dice, name::Name, random::Random, stats::{Stat, StatModifier}};

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
    rupture: Option<u8>,
    add_dmg: u8,
    attack_mod: i8,
    parry_mod: i8,
}

impl Weapon {
    pub fn new(kind: WeaponKind) -> Self {
        match kind {
            WeaponKind::Sword => Self {
                name: String::from("Shitty Sword"),
                kind,
                is_sharp: true,
                add_dmg: 3,
                attack_mod: 0,
                parry_mod: -1,
                rupture: Some(4),
            },
            WeaponKind::Axe => Self {
                name: String::from("Rusty Axe"),
                kind,
                is_sharp: true,
                add_dmg: 3,
                attack_mod: 0,
                parry_mod: -2,
                rupture: Some(3),
            },
            WeaponKind::BattleAxe => Self {
                name: String::from("Coarse Battle Axe"),
                kind,
                is_sharp: true,
                add_dmg: 5,
                attack_mod: -3,
                parry_mod: -4,
                rupture: Some(3),
            },
            WeaponKind::GreatSword => Self {
                name: String::from("Basic Great Sword"),
                kind,
                is_sharp: true,
                add_dmg: 5,
                attack_mod: -3,
                parry_mod: -4,
                rupture: Some(4),
            },
            WeaponKind::Hammer => Self {
                name: String::from("Shitty Hammer"),
                kind,
                is_sharp: false,
                add_dmg: 3,
                attack_mod: 0,
                parry_mod: -2,
                rupture: Some(4),
            },
            WeaponKind::WarHammer => Self {
                name: String::from("Coarse War Hammer"),
                kind,
                is_sharp: false,
                add_dmg: 5,
                attack_mod: -3,
                parry_mod: -4,
                rupture: Some(4),
            },
        }
    }

    pub fn is_sharp(&self) -> bool {
        self.is_sharp
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
    fn modify_stat(&self, base: Stat) -> Stat {
        match base {
            Stat::Attack(_) => base.modify(self.attack_mod),
            Stat::Parry(_) => base.modify(self.parry_mod),
        }
    }
}

impl Name for Weapon {
    fn name(&self) -> &str {
        &self.name
    }
}
