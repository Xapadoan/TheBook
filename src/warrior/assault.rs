use attack::attack_attempt::AttackAttemptResult;
use attack::critical_hit::CriticalHit;
// use attack::{Attack, AttackResult};
use attack::Attack;
use damage_summary::{ApplyDamageSummary, DamageSummary};
use execute_action::ExecuteAction;
use parry::parry_attempt::ParryThreshold;
// use parry::{Parry, ParryResult};
use parry::Parry;
use show_action::ShowAction;

use crate::dice::RollDamage;
use crate::modifiers::ApplyDamageModifier;

use super::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use super::{IsDead, IsUnconscious, Name, TakeDamage, TakeReducedDamage, Warrior};
use super::body::{HasBody, HasMutableBody};
use super::temporary_handicap::parries_miss::CanMissParries;
use super::temporary_handicap::assaults_miss::CanMissAssaults;
use super::duration_damage::MayHaveDurationDamage;

pub mod attack;
pub mod parry;
pub mod damage_summary;
mod show_action;
mod execute_action;
mod clumsiness;

#[derive(Debug)]
pub struct AssaultResult {
    // attack: AttackResult,
    // parry: Option<ParryResult>,
    damage_summary: DamageSummary,
}

impl ApplyDamageSummary for AssaultResult {
    fn apply_damage_summary<T: TakeDamage>(self, assailant: &mut T, victim: &mut T) {
        self.damage_summary.apply_damage_summary(assailant, victim)
    }
}

pub trait Assault {
    fn assault<V: ApplyDamageModifier + Assault + CriticalHit + MayHaveDurationDamage + IsDead + ParryThreshold + TakeReducedDamage + TakeWeapon + MayHaveMutableWeapon + CanMissParries + CanMissAssaults + HasMutableBody + IsUnconscious + MayHaveWeapon + RollDamage + TakeDamage + Name + HasBody>(&mut self, victim: &mut V) -> AssaultResult;
}

impl Assault for Warrior {
    fn assault<V: ApplyDamageModifier + Assault + CriticalHit + MayHaveDurationDamage + IsDead + ParryThreshold + TakeReducedDamage + TakeWeapon + MayHaveMutableWeapon + CanMissParries + CanMissAssaults + HasMutableBody + IsUnconscious + MayHaveWeapon + RollDamage + TakeDamage + Name + HasBody>(&mut self, victim: &mut V) -> AssaultResult {
        let mut attack = self.attack(victim);
        attack.show(self, victim);
        let mut damage_summary = attack.execute(self, victim);
        // let parry = match attack.attack_attempt() {
        match attack.attack_attempt() {
            Some(attack_attempt) => match attack_attempt {
                AttackAttemptResult::Success => {
                    let mut parry_result = victim.parry(self);
                    parry_result.show(self, victim);
                    damage_summary.merge(
                        parry_result.execute(self, victim),
                        false,
                    );
                    Some(parry_result)
                },
                _ => None
            },
            None => None,
        };

        AssaultResult {
            // attack,
            // parry,
            damage_summary,
        }
    }
}
