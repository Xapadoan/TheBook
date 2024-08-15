use serde::{Deserialize, Serialize};

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParrySuccess {}

impl ParrySuccess {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ResolveParrySuccess {
    fn resolve_parry(&self) -> IndividualConsequences {
        IndividualConsequences::no_consequences()
    }
}

impl AssaultConsequencesBuilder for ParrySuccess {
    fn to_consequences(&self, _: & dyn Assailant, _: & dyn Assailant) -> AssaultConsequences {
        AssaultConsequences::new(
            IndividualConsequences::no_consequences(),
            IndividualConsequences::no_consequences(),
        )
    }
}
