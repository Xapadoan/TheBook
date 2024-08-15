use serde::{Deserialize, Serialize};

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};
use super::common_traits::ReduceDamages;

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackSuccess {}

impl AttackSuccess {
    pub fn new() -> Self {
        Self {}
    }
}

impl AssaultConsequencesBuilder for AttackSuccess {
    fn to_consequences(&self, assailant: & dyn Assailant, victim: & dyn Assailant) -> AssaultConsequences {
        let for_assailant = IndividualConsequences::no_consequences();
        let for_victim = victim.resolve_hit(assailant.deal_damages());
        AssaultConsequences::new(for_assailant, for_victim)
    }
}

pub trait ResolveAttackSuccess: ReduceDamages {
    fn resolve_hit(&self, damages: u8) -> IndividualConsequences {
        IndividualConsequences::only_damages(self.reduce_damages(damages))
    }
}
