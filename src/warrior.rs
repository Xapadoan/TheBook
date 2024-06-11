pub mod body;
pub mod protection;

use body::Body;
use body::body_part::BodyPartKind;
use protection::WearProtection;

use crate::dice::Dice;
use crate::dice::RollResult;
use crate::fight_mechanics::critical_hit::CriticalHitResult;
use crate::fight_mechanics::fight_action::ApplyFightActionResult;
use crate::fight_mechanics::fight_action::ShowFightActionResult;
use crate::fight_mechanics::assaults_miss::AssaultsMiss;
use crate::fight_mechanics::parries_miss::ParriesMiss;
use crate::fight_mechanics::parry::ParryAttemptResult;
use crate::fight_mechanics::attack::AttackAttemptResult;
use crate::fight_mechanics::ApplyDamageModifier;
use crate::fight_mechanics::CanMissAssaults;
use crate::fight_mechanics::CanMissParries;
use crate::fight_mechanics::CriticalHitOn;
use crate::fight_mechanics::CriticalParry;
use crate::fight_mechanics::IsDead;
use crate::fight_mechanics::IsUnconscious;
use crate::fight_mechanics::TakeReducibleDamage;
use crate::fight_mechanics::{ParryAttempt, AttackAttempt, TemporaryHandicap};
use crate::fight_mechanics::{ApplyAttackModifier, ApplyParryModifier, RollDamage, TakeDamage};
use crate::weapon::Weapon;

#[derive(Debug)]
pub struct Warrior {
    name: String,
    health: u8,
    attack: u8,
    parry: u8,
    weapon: Weapon,
    assaults_miss: Option<AssaultsMiss>,
    parries_miss: Option<ParriesMiss>,
    is_dead: bool,
    is_unconscious: bool,
    body: Body,
}

impl Warrior {
    pub fn new(name: &str, weapon: Weapon) -> Self {
        Self {
            name: String::from(name),
            health: 30,
            attack: 8,
            parry: 10,
            weapon,
            assaults_miss: None,
            parries_miss: None,
            is_dead: false,
            is_unconscious: false,
            body: Body::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    pub fn attack(&mut self, target: &mut Self) {
        if self.is_dead() || self.is_unconscious() {
            return;
        }
        if self.must_miss_assault() {
            self.miss_assault();
            return;
        }
        println!("{} attacks {}", self.name, target.name);
        let attack_attempt_result = self.attack_attempt();
        attack_attempt_result.show_fight_action_result(self, target);
        attack_attempt_result.apply_fight_action_result(self, target);
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
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
        println!("{} cannot attack because {}", self.name, misses.reason());
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
        let success_threshold = self.weapon.apply_attack_modifier(self.attack);
        match Dice::D6.test_roll(success_threshold) {
            RollResult::CriticalSuccess => AttackAttemptResult::CriticalSuccess,
            RollResult::Success => AttackAttemptResult::Success,
            RollResult::Failure => AttackAttemptResult::Failure,
            RollResult::CriticalFailure => AttackAttemptResult::CriticalFailure
        }
    }
}

impl ParryAttempt for Warrior {
    fn parry_attempt(&self) -> ParryAttemptResult {
        let success_threshold = self.weapon.apply_parry_modifier(self.parry);
        match Dice::D6.test_roll(success_threshold) {
            RollResult::CriticalSuccess => ParryAttemptResult::CriticalSuccess,
            RollResult::Success => ParryAttemptResult::Success,
            RollResult::Failure => ParryAttemptResult::Failure,
            RollResult::CriticalFailure => ParryAttemptResult::CriticalFailure
        }
    }
}

impl CriticalHitOn for Warrior {
    fn critical_hit_on(&self, target: &Warrior) -> CriticalHitResult {
        if self.weapon.is_sharp() {
            CriticalHitResult::roll_sharp(target)
        } else {
            CriticalHitResult::roll_blunt(target)
        }
    }
}

impl CriticalParry for Warrior {}

impl IsDead for Warrior {
    fn is_dead(&self) -> bool {
        self.is_dead
    }

    fn set_dead(&mut self) {
        println!("{} dies", self.name);
        self.is_dead = true;
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
            self.set_dead();
            self.health = 0;
        }
    }
}

impl RollDamage for Warrior {
    fn roll_damage(&self) -> u8 {
        self.weapon.roll_damage()
    }
}

impl ApplyAttackModifier for Warrior {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.body.apply_damage_modifier(base)
    }
}

impl TakeReducibleDamage for Warrior {
    fn take_reduced_damage(&mut self, damage: u8) {
        self.take_damage(self.apply_attack_modifier(damage));
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
