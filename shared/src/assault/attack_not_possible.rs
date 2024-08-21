use serde::{Deserialize, Serialize};

use crate::equipment::weapon::OptionalMutableWeapon;
use crate::health::{IsDead, IsUnconscious};
use crate::temporary_handicap::{OptionalAssaultMisses, TemporaryHandicap};

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};

#[derive(Debug, Serialize, Deserialize)]
pub enum AttackNotPossible {
    AssailantHasNoWeapon,
    AssailantIsDead,
    AssailantIsUnconscious,
    VictimHasNoWeapon,
    VictimIsDead,
    VictimIsUnconscious,
    AssailantMustMissAssault(TemporaryHandicap),
}

impl AssaultConsequencesBuilder for AttackNotPossible {
    fn to_consequences(&self, _: & dyn Assailant, _: & dyn Assailant) -> AssaultConsequences {
        AssaultConsequences::new(
            IndividualConsequences::no_consequences(),
            IndividualConsequences::no_consequences(),
        )
    }
}

pub trait CanBeAttacked:
    OptionalMutableWeapon +
    IsDead +
    IsUnconscious
{
    fn can_be_attacked(&self) -> Option<AttackNotPossible> {
        if self.weapon().is_none() {
            Some(AttackNotPossible::VictimHasNoWeapon)
        } else if self.is_dead() {
            Some(AttackNotPossible::VictimIsDead)
        } else if self.is_unconscious() {
            Some(AttackNotPossible::VictimIsUnconscious)
        } else {
            None
        }
    }
}

pub trait CanAttack:
    OptionalMutableWeapon +
    IsDead +
    IsUnconscious +
    OptionalAssaultMisses
{
    fn can_attack(&self, victim: &dyn Assailant) -> Option<AttackNotPossible> {
        let victim_cant_be_attacked = victim.can_be_attacked();
        if victim_cant_be_attacked.is_some() {
            victim_cant_be_attacked
        } else if self.weapon().is_none() {
            Some(AttackNotPossible::AssailantHasNoWeapon)
        } else if self.is_dead() {
            Some(AttackNotPossible::AssailantIsDead)
        } else if self.is_unconscious() {
            Some(AttackNotPossible::AssailantIsUnconscious)
        } else if let Some(misses) = self.assault_misses() {
            Some(AttackNotPossible::AssailantMustMissAssault(misses.clone()))
        } else {
            None
        }
    }
}
