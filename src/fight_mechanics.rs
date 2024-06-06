pub mod fight_action;
pub mod critical_hit;
pub mod parry;
pub mod attack;

use critical_hit::CriticalHitConsequence;
use parry::ParryAttemptResult;
use attack::AttackAttemptResult;

pub trait IsAlive {
    fn is_alive(&self) -> bool;
}
pub trait TakeDamage {
    fn take_damage(&mut self, damage: u8);
}

pub trait RollDamage {
    fn roll_damage(&self) -> u8;
}

pub trait CriticalHit {
    fn critical_hit(&self) -> CriticalHitConsequence;
}

pub trait AttackAttempt {
    fn attack_attempt(&self) -> AttackAttemptResult;
}

pub trait ParryAttempt {
    fn parry_attempt(&self) -> ParryAttemptResult;
}

pub trait ApplyAttackModifier {
    fn apply_attack_modifier(&self, base: u8) -> u8;
}

pub trait ApplyParryModifier {
    fn apply_parry_modifier(&self, base: u8) -> u8;
}
