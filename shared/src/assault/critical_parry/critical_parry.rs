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
            1 | 2 => CriticalParry::RegularParry,
            3..=5 => CriticalParry::AssailantRepelled,
            6 | 7 => CriticalParry::AssailantTrips,
            8 | 9 => CriticalParry::AssailantFalls,
            10..=12 => CriticalParry::AssailantDropsWeapon,
            13..=15 => CriticalParry::AssailantBreaksWeapon,
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


