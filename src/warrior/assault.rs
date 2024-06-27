use attack::attack_attempt::AttackAttemptResult;
use attack::can_attack::CanAttack;
use attack::{Attack, AttackResult};
use attack::can_be_attacked::CanBeAttacked;
// use assault_damage_summary::AssaultDamageSummary;
// use parry::parry_attempt::ParryAttemptResult;
use parry::{Parry, ParryResult};

use crate::fight_mechanics::{RollDamage, TakeDamage};

use super::weapon::MayHaveWeapon;
use super::{Name, TakeReducedDamage, Warrior};
use super::body::HasBody;

pub mod attack;
mod parry;
mod show_action;
mod execute_action;
// mod assault_damage_summary;

#[derive(Debug)]
pub struct AssaultResult {
    attack: AttackResult,
    parry: Option<ParryResult>,
    // damage_summary: AssaultDamageSummary,
}

pub trait Assault {
    fn assault<V: CanBeAttacked + Parry + HasBody + TakeDamage + TakeReducedDamage + MayHaveWeapon + Name>(&mut self, victim: &mut V) -> AssaultResult;
}

impl<T: Attack + RollDamage + MayHaveWeapon + TakeDamage + Name> Assault for T {
    fn assault<V: CanBeAttacked + Parry + HasBody + TakeDamage + TakeReducedDamage + MayHaveWeapon + Name>(&mut self, victim: &mut V) -> AssaultResult {
        let attack = self.attack(victim);
        let parry = match attack.attack_attempt() {
            Some(attack_attempt) => match attack_attempt {
                AttackAttemptResult::Success => Some(victim.parry(self)),
                _ => None
            },
            None => None,
        };
        AssaultResult {
            attack,
            parry
        }
    }
}
