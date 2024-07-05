pub mod body;
pub mod protection;
pub mod stats;
pub mod assault;
pub mod weapon;
pub mod temporary_handicap;
pub mod duration_damage;
mod warrior_name;

use assault::attack::attack_attempt::AttackThreshold;
use assault::parry::parry_attempt::ParryThreshold;
use body::Body;
use body::body_part::BodyPartKind;
use body::HasBody;
use body::HasMutableBody;
use protection::WearProtection;
use serde::{Serialize, Deserialize};
use stats::StatModifier;
use stats::StatsManager;
use stats::Stat;
use temporary_handicap::TemporaryHandicap;
use uuid::Uuid;
use weapon::GiveWeapon;
use weapon::MayHaveMutableWeapon;
use weapon::MayHaveWeapon;
use weapon::TakeWeapon;
use weapon::Weapon;
use temporary_handicap::parries_miss::{CanMissParries, ParriesMiss};
use temporary_handicap::assaults_miss::{CanMissAssaults, AssaultsMiss};
use duration_damage::{DurationDamage, MayHaveDurationDamage};
use warrior_name::WarriorName;

use crate::dice::{RollDamage, Dice};
use crate::gen_random::GenRandom;
use crate::modifiers::{ApplyDamageModifier, Modifier};
use crate::name::HasName;
use crate::repository::main::UniqueEntity;

pub trait IsDead {
    fn is_dead(&self) -> bool;
}

pub trait IsUnconscious {
    fn is_unconscious(&self) -> bool;
    fn set_unconscious(&mut self);
}

pub trait TakeDamage {
    fn take_damage(&mut self, damage: u8);
}

pub trait TakeReducedDamage {
    fn take_reduced_damage(&mut self, damage: u8);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warrior {
    uuid: Uuid,
    name: WarriorName,
    stats_manager: StatsManager,
    health: u8,
    weapon: Option<Weapon>,
    assaults_miss: Option<AssaultsMiss>,
    parries_miss: Option<ParriesMiss>,
    is_unconscious: bool,
    body: Body,
    duration_damages: Vec<DurationDamage>,
}

impl Warrior {
    fn new(uuid: Uuid, name: WarriorName) -> Self {
        Self {
            uuid,
            name,
            stats_manager: StatsManager::new(),
            health: 30,
            weapon: None,
            assaults_miss: None,
            parries_miss: None,
            is_unconscious: false,
            body: Body::new(),
            duration_damages: Vec::new(),
        }
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    pub fn apply_duration_damages(&mut self, time_elapsed: u32) {
        let mut damages = 0;
        for duration_damage in &mut self.duration_damages {
            if duration_damage.should_take_duration_damage(time_elapsed) {
                damages += duration_damage.roll_damage();
                duration_damage.add_hit();
                println!("{} took duration damage because {}", self.name, duration_damage.reason());
            }
        }
        self.take_damage(damages)
    }
}

impl CanMissParries for Warrior {
    fn must_miss_parry(&self) -> bool {
        self.parries_miss.is_some()
    }

    fn must_miss_parry_reason(&self) -> &String {
        self.parries_miss.as_ref().unwrap().reason()
    }

    fn miss_parry(&mut self) {
        let misses = self.parries_miss.as_mut().unwrap();
        misses.decrement_turns_count();
        println!("{} cannot parry because {}", self.name, misses.reason());
        if misses.turns_left() == 0 {
            self.parries_miss = None;
        }
    }

    fn will_miss_parries(&mut self, misses: ParriesMiss) {
        self.parries_miss = Some(misses);
    }
}

impl CanMissAssaults for Warrior {
    fn must_miss_assault(&self) -> bool {
        self.assaults_miss.is_some()
    }

    fn must_miss_assault_reason(&self) -> &String {
        self.assaults_miss.as_ref().unwrap().reason()
    }

    fn miss_assault(&mut self) {
        dbg!(&self.assaults_miss);
        let misses = self.assaults_miss.as_mut().unwrap();
        misses.decrement_turns_count();
        dbg!(&misses);
        if misses.turns_left() == 0 {
            self.assaults_miss = None;
        }
    }

    fn will_miss_assault(&mut self, misses: AssaultsMiss) {
        self.assaults_miss = Some(misses)
    }
}

impl IsUnconscious for Warrior {
    fn is_unconscious(&self) -> bool {
        self.is_unconscious
    }

    fn set_unconscious(&mut self) {
        println!("{} falls unconscious", self.name);
        self.is_unconscious = true;
    }
}

impl TakeDamage for Warrior {
    fn take_damage(&mut self, dmg: u8) {
        if self.health > dmg {
            self.health -= dmg;
            if self.health < 5 {
                self.set_unconscious();
            }
        } else {
            self.health = 0;
        }
    }
}

impl RollDamage for Warrior {
    fn roll_damage(&self) -> u8 {
        match &self.weapon {
            Some(weapon) => weapon.roll_damage(),
            None => Modifier::new(-2).apply(Dice::D6.roll()),
        }
    }
}

impl ApplyDamageModifier for Warrior {
    fn apply_damage_modifier(&self, base: u8) -> u8 {
        self.body.apply_damage_modifier(base)
    }
}

impl TakeReducedDamage for Warrior {
    fn take_reduced_damage(&mut self, damage: u8) {
        self.take_damage(self.apply_damage_modifier(damage));
    }
}

impl WearProtection for Warrior {
    fn can_wear_protection(&self, protection: &protection::Protection, body_part: BodyPartKind) -> bool {
        self.body.can_wear_protection(protection, body_part)
    }

    fn wear_protection(&mut self, protection: protection::Protection, body_part: BodyPartKind) {
        self.body.wear_protection(protection, body_part)
    }
}

impl StatModifier for Warrior {
    fn modify_stat(&self, base: Stat) -> Stat {
        let mut stat = base;
        stat = match &self.weapon {
            Some(weapon) => weapon.modify_stat(stat),
            None => match stat {
                Stat::Attack(attack) => Stat::Attack(Modifier::new(-4).apply(attack)),
                Stat::Parry(_) => stat,
            }
        };
        stat = self.body.modify_stat(stat);
        stat
    }
}

impl MayHaveWeapon for Warrior {
    fn weapon(&self) -> Option<&Weapon> {
        self.weapon.as_ref()
    }
}

impl MayHaveMutableWeapon for Warrior {
    fn weapon_mut(&mut self) -> Option<&mut Weapon> {
        self.weapon.as_mut()
    }
}

impl TakeWeapon for Warrior {
    fn take_weapon(&mut self) -> Option<Weapon> {
        if self.weapon.is_none() {
            None
        } else {
            self.weapon.take()
        }
    }
}

impl GiveWeapon for Warrior {
    fn give_weapon(&mut self, weapon: Weapon) {
        self.weapon = Some(weapon)
    }
}

impl IsDead for Warrior {
    fn is_dead(&self) -> bool {
        self.health < 1
    }
}

impl HasBody for Warrior {
    fn body(&self) -> &Body {
        &self.body
    }
}

impl HasMutableBody for Warrior {
    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

impl HasName for Warrior {
    fn name(&self) -> &WarriorName {
        &self.name
    }
}

impl MayHaveDurationDamage for Warrior {
    fn add_duration_damage(&mut self, reason: String, start_at: u32) {
        self.duration_damages.push(DurationDamage::new(reason, start_at))
    }
}

impl AttackThreshold for Warrior {
    fn attack_threshold(&self) -> u8 {
        Stat::consume(self.modify_stat(self.stats_manager.attack_stat()))
    }
}

impl ParryThreshold for Warrior {
    fn parry_threshold(&self) -> u8 {
        Stat::consume(self.modify_stat(self.stats_manager.parry_stat()))
    }
}

impl GenRandom for Warrior {
    fn gen_random() -> Self {
        Self::new(Uuid::new_v4(), WarriorName::gen_random())
    }
}

impl UniqueEntity for Warrior {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}