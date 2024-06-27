pub mod body;
pub mod protection;
pub mod stats;
pub mod assault;
pub mod weapon;

use assault::Assault;
use body::Body;
use body::body_part::BodyPartKind;
use body::HasBody;
use body::HasMutableBody;
use protection::WearProtection;
use stats::StatModifier;
use stats::StatsManager;
use stats::Stat;
use weapon::GiveWeapon;
// use weapon::CanHaveWeapon;
use weapon::MayHaveMutableWeapon;
use weapon::MayHaveWeapon;
use weapon::TakeWeapon;
use weapon::Weapon;

use crate::dice::Dice;
use crate::dice::TestRollResult;
use crate::fight_mechanics::duration_damage::DurationDamage;
// use crate::fight_mechanics::fight_action::ExecuteFightActionResult;
// use crate::fight_mechanics::fight_action::ShowFightActionResult;
use crate::fight_mechanics::assaults_miss::AssaultsMiss;
use crate::fight_mechanics::duration_damage::MayHaveDurationDamage;
use crate::fight_mechanics::parries_miss::ParriesMiss;
use crate::fight_mechanics::parry::ParryAttemptResult;
use crate::fight_mechanics::attack::AttackAttemptResult;
use crate::fight_mechanics::ApplyDamageModifier;
use crate::fight_mechanics::CanMissAssaults;
use crate::fight_mechanics::CanMissParries;
use crate::fight_mechanics::{ParryAttempt, AttackAttempt, TemporaryHandicap};
use crate::fight_mechanics::{RollDamage, TakeDamage};
use crate::modifiers::Modifier;

pub trait IsDead {
    fn is_dead(&self) -> bool;
}

pub trait IsUnconscious {
    fn is_unconscious(&self) -> bool;
    fn set_unconscious(&mut self);
}

pub trait TakeReducedDamage {
    fn take_reduced_damage(&mut self, damage: u8);
}

pub trait Name {
    fn name(&self) -> &String;
}

#[derive(Debug)]
pub struct Warrior {
    name: String,
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
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    // pub fn attack(&mut self, target: &mut Self) {
    //     let pre_assault_result = self.can_start_assault();
    //     if !pre_assault_result.can_start() {
    //         match pre_assault_result.reason().unwrap() {
    //             CantAttackReason::MustMissAssault => {
    //                 self.miss_assault();
    //                 println!("{} can't attack because {}", self.name, self.assaults_miss.as_ref().unwrap().reason())
    //             },
    //             _ => {}
    //         }
    //         return;
    //     }
    //     println!("{} attacks {}", self.name, target.name);
    //     let mut attack_attempt_result = self.attack_attempt();
    //     attack_attempt_result.show_fight_action_result(self, target);
    //     attack_attempt_result.execute(self, target);
    // }

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

    pub fn add_duration_damage(&mut self, reason: String, start_at: u32) {
        self.duration_damages.push(DurationDamage::new(reason, start_at))
    }
}

impl CanMissParries for Warrior {
    fn must_miss_parry(&self) -> bool {
        self.parries_miss.is_some()
    }

    fn miss_parry(&mut self) {
        let misses = self.parries_miss.as_mut().unwrap();
        misses.decrement_count();
        println!("{} cannot parry because {}", self.name, misses.reason());
        if misses.count() == 0 {
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

    fn miss_assault(&mut self) {
        let misses = self.assaults_miss.as_mut().unwrap();
        misses.decrement_count();
        if misses.count() == 0 {
            self.assaults_miss = None;
        }
    }

    fn will_miss_assault(&mut self, misses: AssaultsMiss) {
        self.assaults_miss = Some(misses)
    }
}

impl AttackAttempt for Warrior {
    fn attack_attempt(&self) -> AttackAttemptResult {
        let success_threshold = self.modify_stat(self.stats_manager.attack_stat());
        match Dice::D6.test_roll(Stat::consume(success_threshold)) {
            TestRollResult::CriticalSuccess => AttackAttemptResult::CriticalSuccess,
            TestRollResult::Success => AttackAttemptResult::Success,
            TestRollResult::Failure => AttackAttemptResult::Failure,
            TestRollResult::CriticalFailure => AttackAttemptResult::CriticalFailure
        }
    }
}

impl ParryAttempt for Warrior {
    fn parry_attempt(&self) -> ParryAttemptResult {
        if self.weapon.is_none() {
            return ParryAttemptResult::Failure;
        }
        let success_threshold = self.modify_stat(self.stats_manager.parry_stat());
        match Dice::D6.test_roll(Stat::consume(success_threshold)) {
            TestRollResult::CriticalSuccess => ParryAttemptResult::CriticalSuccess,
            TestRollResult::Success => ParryAttemptResult::Success,
            TestRollResult::Failure => ParryAttemptResult::Failure,
            TestRollResult::CriticalFailure => ParryAttemptResult::CriticalFailure
        }
    }
}

// impl CriticalParry for Warrior {}

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

impl Name for Warrior {
    fn name(&self) -> &String {
        &self.name
    }
}

impl MayHaveDurationDamage for Warrior {
    fn add_duration_damage(&mut self, reason: String, start_at: u32) {
        self.duration_damages.push(DurationDamage::new(reason, start_at))
    }
}
