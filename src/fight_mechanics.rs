pub mod fight_action;
pub mod critical_hit;
pub mod critical_parry;
pub mod parry;
pub mod attack;
pub mod assaults_miss;
pub mod parries_miss;

use critical_hit::CriticalHitResult;
use assaults_miss::AssaultsMiss;
use parries_miss::ParriesMiss;
use parry::ParryAttemptResult;
use attack::AttackAttemptResult;

use crate::warrior::Warrior;

pub trait IsDead {
    fn is_dead(&self) -> bool;
    fn set_dead(&mut self);
}

pub trait IsUnconscious {
    fn is_unconscious(&self) -> bool;
    fn set_unconscious(&mut self);
}
pub trait TakeDamage {
    fn take_damage(&mut self, damage: u8);
}

pub trait TakeReducibleDamage {
    fn take_reduced_damage(&mut self, damage: u8);
}

pub trait RollDamage {
    fn roll_damage(&self) -> u8;
}

pub trait CriticalHitOn {
    fn critical_hit_on(&self, target: &Warrior) -> CriticalHitResult;
}

pub trait AttackAttempt {
    fn attack_attempt(&self) -> AttackAttemptResult;
}

pub trait ParryAttempt {
    fn parry_attempt(&self) -> ParryAttemptResult;
}

pub trait ApplyDamageModifier {
    fn apply_damage_modifier(&self, base: u8) -> u8;
}

pub trait TemporaryHandicap {
    fn new(count: u8, reason: String) -> Self;
    fn decrement_count(&mut self);
    fn count(&self) -> u8;
    fn reason(&self) -> &String;
}

pub trait CanMissAssaults {
    fn must_miss_assault(&self) -> bool;
    fn will_miss_assault(&mut self, misses: AssaultsMiss);
    fn miss_assault(&mut self);
}

pub trait CanMissParries {
    fn must_miss_parry(&self) -> bool;
    fn will_miss_parries(&mut self, misses: ParriesMiss);
    fn miss_parry(&mut self);
}
