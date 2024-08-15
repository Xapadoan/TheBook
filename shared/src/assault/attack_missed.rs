use serde::{Deserialize, Serialize};

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackMissed {}

impl AttackMissed {
    pub fn new() -> Self {
        Self {}
    }
}

impl AssaultConsequencesBuilder for AttackMissed {
    fn to_consequences(&self, _: & dyn Assailant, _: & dyn Assailant) -> AssaultConsequences {
        AssaultConsequences::new(
            IndividualConsequences::no_consequences(),
            IndividualConsequences::no_consequences(),
        )
    }
}
