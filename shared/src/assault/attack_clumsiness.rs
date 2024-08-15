use serde::{Deserialize, Serialize};

use crate::random::Random;

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};
use super::clumsiness::{Clumsiness, ResolveClumsiness};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackClumsiness {
    kind: Clumsiness,
}

impl AttackClumsiness {
    pub fn kind(&self) -> &Clumsiness {
        &self.kind
    }
}

impl Random for AttackClumsiness {
    fn random() -> Self {
        Self { kind: Clumsiness::random() }
    }
}

pub trait ResolveAttackClumsiness: ResolveClumsiness {
    fn resolve_attack_clumsiness(&self, clumsiness: Clumsiness) -> IndividualConsequences {
        self.resolve_clumsiness(
            clumsiness,
            IndividualConsequences::no_consequences(),
        )
    }
}

impl AssaultConsequencesBuilder for AttackClumsiness {
    fn to_consequences(&self, assailant: & dyn Assailant, _: & dyn Assailant) -> AssaultConsequences {
        let for_assailant = assailant.resolve_attack_clumsiness(self.kind.clone());
        let for_victim = IndividualConsequences::no_consequences();
        AssaultConsequences::new(for_assailant, for_victim)
    }
}
