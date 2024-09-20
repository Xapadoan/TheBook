use serde::{Deserialize, Serialize};

use crate::assault::assailant::Assailant;
use crate::assault::assault_consequence::{AssaultConsequences, IndividualConsequences, AssaultConsequencesBuilder};
use crate::dice::Dice;
use crate::random::Random;

#[derive(Debug, Serialize, Deserialize)]
pub enum CriticalParry {
    RegularParry,
    AssailantRepelled,
    AssailantTrips,
    AssailantFalls,
    AssailantDropsWeapon,
    AssailantBreaksWeapon,
    AssailantHit,
    AssailantCriticalHit,
    AssailantSelfCriticalHit,
}

impl Random for CriticalParry {
    fn random() -> Self {
        match Dice::D20.roll() {
            1..=3 => CriticalParry::RegularParry,
            4..=6 => CriticalParry::AssailantRepelled,
            7..=9 => CriticalParry::AssailantTrips,
            10 | 11 => CriticalParry::AssailantFalls,
            12..=13 => CriticalParry::AssailantDropsWeapon,
            14 | 15 => CriticalParry::AssailantBreaksWeapon,
            16..=18 => CriticalParry::AssailantHit,
            19 => CriticalParry::AssailantCriticalHit,
            20 => CriticalParry::AssailantSelfCriticalHit,
            other => panic!("D20 roll resulted in {other}"),
        }
    }
}

impl AssaultConsequencesBuilder for CriticalParry {
    fn to_consequences(&self, assailant: & dyn Assailant, victim: & dyn Assailant) -> AssaultConsequences {
        let for_victim = IndividualConsequences::no_consequences();
        let for_assailant = assailant.resolve_critical_parry(self, victim);
        AssaultConsequences::new(for_assailant, for_victim)
    }
}


