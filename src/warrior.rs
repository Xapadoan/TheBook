use crate::dice::Dice;
use crate::dice::RollResult;
use crate::fight_mechanics::fight_action::ApplyFightActionResult;
use crate::fight_mechanics::fight_action::ShowFightActionResult;
use crate::fight_mechanics::critical_hit::CriticalHitConsequence;
use crate::fight_mechanics::parry::ParryAttemptResult;
use crate::fight_mechanics::attack::AttackAttemptResult;
use crate::fight_mechanics::CriticalHit;
use crate::fight_mechanics::IsAlive;
use crate::fight_mechanics::{ParryAttempt, AttackAttempt};
use crate::fight_mechanics::{ApplyAttackModifier, ApplyParryModifier, RollDamage, TakeDamage};
use crate::weapon::Weapon;

#[derive(Debug)]
pub struct Warrior {
    name: String,
    health: u8,
    attack: u8,
    parry: u8,
    weapon: Weapon,
}

impl Warrior {
    pub fn new(name: &str, weapon: Weapon) -> Self {
        Self {
            name: String::from(name),
            health: 30,
            attack: 8,
            parry: 10,
            weapon,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    pub fn attack(&mut self, target: &mut Self) {
        println!("{} attacks {}", self.name, target.name);
        let attack_attempt_result = self.attack_attempt();
        attack_attempt_result.show_fight_action_result(self, target);
        attack_attempt_result.apply_fight_action_result(self, target);
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
    fn critical_hit(&self) -> CriticalHitConsequence {
        self.weapon.critical_hit()
    }
}

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
