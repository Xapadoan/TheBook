pub mod critical_hit_option;

use critical_hit_option::CriticalConsequence;

pub trait TakeDamage {
    fn take_damage(&mut self, damage: u8);
}

pub trait RollDamage {
    fn roll_damage(&self) -> u8;
}

pub trait CriticalHit {
    fn critical_hit(&self) -> CriticalConsequence;
}

pub trait ApplyAttackModifier {
    fn apply_attack_modifier(&self, base: u8) -> u8;
}

pub trait ApplyParryModifier {
    fn apply_parry_modifier(&self, base: u8) -> u8;
}
