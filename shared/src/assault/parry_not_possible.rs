use serde::{Deserialize, Serialize};

use crate::equipment::weapon::OptionalMutableWeapon;
use crate::health::{IsDead, IsUnconscious};
use crate::temporary_handicap::{OptionalParryMisses, TemporaryHandicap};

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};

#[derive(Debug, Serialize, Deserialize)]
pub enum ParryNotPossible {
    HasNoWeapon,
    IsDead,
    IsUnconscious,
    MustMissParry(TemporaryHandicap),
}

impl AssaultConsequencesBuilder for ParryNotPossible {
    fn to_consequences(&self, _: & dyn Assailant, _: & dyn Assailant) -> AssaultConsequences {
        AssaultConsequences::new(
            IndividualConsequences::no_consequences(),
            IndividualConsequences::no_consequences(),
        )
    }
}

pub trait CanParry:
    OptionalMutableWeapon +
    IsDead +
    IsUnconscious +
    OptionalParryMisses
{
    fn can_parry(&self) -> Option<ParryNotPossible> {
        if self.weapon().is_none() {
            Some(ParryNotPossible::HasNoWeapon)
        } else if self.is_dead() {
            Some(ParryNotPossible::IsDead)
        } else if self.is_unconscious() {
            Some(ParryNotPossible::IsUnconscious)
        } else if let Some(misses) = self.parry_misses() {
            Some(ParryNotPossible::MustMissParry(misses.clone()))
        } else {
            None
        }
    }
}
