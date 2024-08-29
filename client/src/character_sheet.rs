use shared::assault::attack_attempt::AttackThreshold;
use shared::assault::parry_attempt::ParryThreshold;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::experience::Experience;
use shared::health::{Health, MutableHealth};
use shared::name::Name;
use shared::stats::{Stat, StatModifier, Stats, StatsManager};
use shared::warrior::body::{Body, HasBody};
use shared::warrior::Warrior;

use super::show::ShowSelf;

pub struct CharacterSheet<'a> {
    name: &'a str,
    health: &'a Health,
    body: &'a Body,
    stats: &'a StatsManager,
    weapon: &'a Option<Weapon>,
    experience: u64,
}

impl<'a> CharacterSheet<'a> {
    pub fn new(warrior: &'a Warrior) -> Self {
        Self {
            name: warrior.name(),
            health: warrior.health(),
            body: warrior.body(),
            stats: warrior.stats(),
            weapon: warrior.weapon(),
            experience: warrior.xp(),
        }
    }
}

impl<'a> ShowSelf for CharacterSheet<'a> {
    fn show_self(&self) -> String {
        let weapon_str = if let Some(weapon) = self.weapon {
            weapon.show_self()
        } else {
            String::from("None")
        };
        format!(
            "{}\n{}/{}\nAT: {} PRD: {}\n{}\nLevel: {} ({}xp)",
            self.name,
            self.health.current(),
            self.health.max(),
            self.attack_threshold(),
            self.parry_threshold(),
            weapon_str,
            self.level(),
            self.xp(),
        )
    }
}

impl<'a> AttackThreshold for CharacterSheet<'a> {
    fn attack_threshold(&self) -> u8 {
        let mut attack = Stat::Attack(self.stats.attack().value());
        if let Some(weapon) = self.weapon {
            attack = weapon.modify_stat(attack);
        }
        attack = self.body.modify_stat(attack);
        attack.value()
    }
}

impl<'a> ParryThreshold for CharacterSheet<'a> {
    fn parry_threshold(&self) -> u8 {
        let mut parry = Stat::Parry(self.stats.parry().value());
        if let Some(weapon) = self.weapon {
            parry = weapon.modify_stat(parry);
        }
        parry = self.body.modify_stat(parry);
        parry.value()
    }
}

impl<'a> Experience for CharacterSheet<'a> {
    fn xp(&self) -> u64 {
        self.experience
    }
}
