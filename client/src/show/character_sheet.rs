use shared::assault::attack_attempt::AttackThreshold;
use shared::assault::parry_attempt::ParryThreshold;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::experience::Experience;
use shared::health::{Health, MutableHealth};
use shared::name::Name;
use shared::stats::{Stat, StatModifier, Stats, StatsManager};
use shared::warrior::body::{Body, HasBody};
use shared::warrior::Warrior;

use super::ShowSelf;

pub struct CharacterSheet<'a> {
    name: &'a str,
    health: &'a Health,
    body: &'a Body,
    stats: &'a StatsManager,
    weapon: &'a Option<Weapon>,
    experience: u64,
    level: u8,
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
            level: warrior.level(),
        }
    }
}

impl<'a> ShowSelf for CharacterSheet<'a> {
    fn show_self(&self) -> String {
        let mut stat_modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(self.body)];
        let weapon_str = if let Some(weapon) = self.weapon {
            stat_modifiers.push(Box::new(weapon));
            weapon.show_self()
        } else {
            String::from("None")
        };
        format!(
            "{}\nHP: {}/{}\nWeapon: {}\nAT: {}\tPRD: {}\nCOU: {} ({})\tDEX: {} ({})\tSTR: {} ({})\nLevel: {} ({}xp)",
            self.name,
            self.health.current(),
            self.health.max(),
            weapon_str,
            self.attack_threshold(),
            self.parry_threshold(),
            self.stats.stat(&[], &Stat::Courage(0)).value(),
            self.stats.stat(&stat_modifiers, &Stat::Courage(0)).value(),
            self.stats.stat(&[], &Stat::Dexterity(0)).value(),
            self.stats.stat(&stat_modifiers, &Stat::Dexterity(0)).value(),
            self.stats.stat(&[], &Stat::Strength(0)).value(),
            self.stats.stat(&stat_modifiers, &Stat::Strength(0)).value(),
        ).as_str();
            self.level(),
            self.xp(),
        )
    }
}

impl<'a> AttackThreshold for CharacterSheet<'a> {
    fn attack_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(self.body)];
        if let Some(weapon) = self.weapon {
            modifiers.push(Box::new(weapon));
        }
        self.stats.stat(&modifiers, &Stat::Attack(0)).value()
    }
}

impl<'a> ParryThreshold for CharacterSheet<'a> {
    fn parry_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(self.body)];
        if let Some(weapon) = self.weapon {
            modifiers.push(Box::new(weapon));
        }
        self.stats.stat(&modifiers, &Stat::Parry(0)).value()
    }
}

impl<'a> Experience for CharacterSheet<'a> {
    fn xp(&self) -> u64 {
        self.experience
    }
    fn level(&self) -> u8 {
        self.level
    }
}
