use serde::{Deserialize, Serialize};

use crate::random::Random;

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};
use super::attack_success::ResolveAttackSuccess;
use super::clumsiness::{Clumsiness, ResolveClumsiness};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParryClumsiness {
    kind: Clumsiness,
}

impl ParryClumsiness {
    pub fn kind(&self) -> &Clumsiness {
        &self.kind
    }
}

impl Random for ParryClumsiness {
    fn random() -> Self {
        Self {
            kind: Clumsiness::random()
        }
    }
}

pub trait ResolveParryClumsiness:
    ResolveClumsiness +
    ResolveAttackSuccess
{
    fn resolve_parry_clumsiness(&self, clumsiness: Clumsiness, damages: u8) -> IndividualConsequences {
        self.resolve_clumsiness(
            clumsiness,
            self.resolve_hit(damages)
        )
    }
}

impl AssaultConsequencesBuilder for ParryClumsiness {
    fn to_consequences(&self, assailant: & dyn Assailant, _: & dyn Assailant) -> AssaultConsequences {
        let for_assailant = IndividualConsequences::no_consequences();
        let damages = assailant.deal_damages();
        let for_victim = assailant.resolve_parry_clumsiness(self.kind.clone(), damages);
        AssaultConsequences::new(for_assailant, for_victim)
    }
}
