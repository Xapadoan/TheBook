use crate::dice::Dice;
use crate::dice::RollResult;
use crate::fight_mechanics::critical_hit::CriticalHitResult;
use crate::fight_mechanics::fight_action::ApplyFightActionResult;
use crate::fight_mechanics::fight_action::ShowFightActionResult;
use crate::fight_mechanics::assaults_miss::AssaultsMiss;
use crate::fight_mechanics::parries_miss::ParriesMiss;
use crate::fight_mechanics::parry::ParryAttemptResult;
use crate::fight_mechanics::attack::AttackAttemptResult;
use crate::fight_mechanics::CanMissAssaults;
use crate::fight_mechanics::CanMissParries;
use crate::fight_mechanics::CriticalHit;
use crate::fight_mechanics::CriticalParry;
use crate::fight_mechanics::IsAlive;
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
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    pub fn attack(&mut self, target: &mut Self) {
        if self.must_miss_assault() {
            self.miss_assault();
            return;
        }
        println!("{} attacks {}", self.name, target.name);
        let attack_attempt_result = self.attack_attempt();
        attack_attempt_result.show_fight_action_result(self, target);
        attack_attempt_result.apply_fight_action_result(self, target);
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

impl CriticalHit for Warrior {
    fn critical_hit(&self) -> CriticalHitResult {
        self.weapon.critical_hit()
    }
}

impl CriticalParry for Warrior {}

impl IsAlive for Warrior {
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl TakeDamage for Warrior {
    fn take_damage(&mut self, dmg: u8) {
        if self.health > dmg {
            self.health -= dmg;
        } else {
            self.health = 0;
        }
    }
}

impl RollDamage for Warrior {
    fn roll_damage(&self) -> u8 {
        self.weapon.roll_damage()
    }
}
