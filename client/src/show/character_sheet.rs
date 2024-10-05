use shared::assault::attack_attempt::AttackThreshold;
use shared::assault::parry_attempt::ParryThreshold;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::experience::Experience;
use shared::health::{Health, MutableHealth};
use shared::name::Name;
use shared::stats::{StatKind, StatModifier, Stats, StatsManager};
use shared::warrior::body::{Body, HasBody};
use shared::warrior::Warrior;

use super::{ShowSelf, ShowSelfExtended};

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
        if let Some(weapon) = self.weapon {
            stat_modifiers.push(Box::new(weapon));
        }
        let mut str = String::new();
        str += format!(
            "{}\nHP: {}/{}",
            self.name,
            self.health.current(),
            self.health.max(),
        ).as_str();
        str += format!(
            "\nWeapon: {}",
            self.weapon.show_self(),
        ).as_str();
        str += format!(
            "\nAT: {}\tPRD: {}\nCOU: {} ({})\tDEX: {} ({})\tSTR: {} ({})",
            self.attack_threshold(),
            self.parry_threshold(),
            self.stats.nat_stat(&StatKind::Courage).value(),
            self.stats.stat(&stat_modifiers, &StatKind::Courage).value(),
            self.stats.nat_stat(&StatKind::Dexterity).value(),
            self.stats.stat(&stat_modifiers, &StatKind::Dexterity).value(),
            self.stats.nat_stat(&StatKind::Strength).value(),
            self.stats.stat(&stat_modifiers, &StatKind::Strength).value(),
        ).as_str();
        str += format!(
            "\nLevel: {} ({}xp)",
            self.level(),
            self.xp(),
        ).as_str();

        str
    }
}

impl<'a> AttackThreshold for CharacterSheet<'a> {
    fn attack_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(self.body)];
        if let Some(weapon) = self.weapon {
            modifiers.push(Box::new(weapon));
        }
        self.stats.stat(&modifiers, &StatKind::Attack).value()
    }
}

impl<'a> ParryThreshold for CharacterSheet<'a> {
    fn parry_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(self.body)];
        if let Some(weapon) = self.weapon {
            modifiers.push(Box::new(weapon));
        }
        self.stats.stat(&modifiers, &StatKind::Parry).value()
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

impl<'a> ShowSelfExtended for CharacterSheet<'a> {
    fn show_self_extended(&self) -> String {
        let mut str = String::new();
        str += self.name;
        str += format!(
            "\nHP: {}/{}",
            self.health.current(),
            self.health.max(),
        ).as_str();
        str += format!("\nWeapon: {}", self.weapon.show_self_extended()).as_str();
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(self.body)];
        if let Some(weapon) = self.weapon {
            modifiers.push(Box::new(weapon));
        }
        str += format!("\n\n{}", show_stats_with_modifiers(self.stats, &modifiers, &StatKind::Attack)).as_str();
        str += format!("\n{}", show_stats_with_modifiers(self.stats, &modifiers, &StatKind::Parry)).as_str();
        str += format!("\n{}", show_stats_with_modifiers(self.stats, &modifiers, &StatKind::Courage)).as_str();
        str += format!("\n{}", show_stats_with_modifiers(self.stats, &modifiers, &StatKind::Dexterity)).as_str();
        str += format!("\n{}", show_stats_with_modifiers(self.stats, &modifiers, &StatKind::Strength)).as_str();
        
        str += format!("\n\n{}", self.body.show_self_extended()).as_str();

        str += format!(
            "\n\nLevel: {} ({}xp)",
            self.level(),
            self.xp(),
        ).as_str();

        str
    }
}

fn show_stats_with_modifiers(manager: &StatsManager, modifiers: &[Box<&dyn StatModifier>], stat: &StatKind) -> String {
    let mut str = match stat {
        StatKind::Attack => "AT",
        StatKind::Parry => "PRD",
        StatKind::Courage => "COU",
        StatKind::Dexterity => "DEX",
        StatKind::Strength => "STR",
    }.to_string();
    str += format!(": {}", manager.stat(modifiers, stat).value()).as_str();
    str += format!(" ({}", manager.nat_stat(stat).value()).as_str();
    for modifier in modifiers {
        let value = modifier.value(stat);
        let sign = if value < 0 { "-" } else { "+" };
        str += format!(" {sign} {}", value.abs()).as_str();
    }
    str += ")";

    str
}
