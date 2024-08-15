use serde::{Deserialize, Serialize};

use crate::equipment::weapon::OptionalMutableWeapon;
use crate::health::{IsDead, IsUnconscious};
use crate::temporary_handicap::{OptionalAssaultMisses, TemporaryHandicap};

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};

#[derive(Debug, Serialize, Deserialize)]
pub enum NotPossible {
    AssailantHasNoWeapon,
    AssailantIsDead,
    AssailantIsUnconscious,
    VictimHasNoWeapon,
    VictimIsDead,
    VictimIsUnconscious,
    AssailantMustMissAssault(TemporaryHandicap),
}

impl AssaultConsequencesBuilder for NotPossible {
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
    fn can_be_attacked(&self) -> Option<NotPossible> {
        if self.weapon().is_none() {
            Some(NotPossible::VictimHasNoWeapon)
        } else if self.is_dead() {
            Some(NotPossible::VictimIsDead)
        } else if self.is_unconscious() {
            Some(NotPossible::VictimIsUnconscious)
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
    fn can_attack(&self, victim: &dyn Assailant) -> Option<NotPossible> {
        let victim_cant_be_attacked = victim.can_be_attacked();
        if !victim_cant_be_attacked.is_some() {
            victim_cant_be_attacked
        } else if self.weapon().is_none() {
            Some(NotPossible::AssailantHasNoWeapon)
        } else if self.is_dead() {
            Some(NotPossible::AssailantIsDead)
        } else if self.is_unconscious() {
            Some(NotPossible::AssailantIsUnconscious)
        } else if let Some(misses) = self.assault_misses() {
            Some(NotPossible::AssailantMustMissAssault(
                TemporaryHandicap::new(misses.count()))
            )
        } else {
            None
        }
    }
}
