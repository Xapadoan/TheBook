use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assault::assailant::Assailant;
use crate::assault::assault_summary::AssaultSummary;
use crate::assault::end_turn_consequences::EndTurnConsequences;

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnSummary {
    first_assailant_uuid: Uuid,
    second_assailant_uuid: Uuid,
    assaults: [AssaultSummary; 2],
    first_assailant_turn_end: EndTurnConsequences,
    second_assailant_turn_end: EndTurnConsequences,
}

impl TurnSummary {
    pub fn assaults(&self) -> &[AssaultSummary; 2] {
        &self.assaults
    }

    pub fn first_assailant_turn_end(&self) -> &EndTurnConsequences {
        &self.first_assailant_turn_end
    }

    pub fn second_assailant_turn_end(&self) -> &EndTurnConsequences {
        &self.second_assailant_turn_end
    }

    // server only
    pub fn new(
        first_assailant: &mut dyn Assailant,
        second_assailant: &mut dyn Assailant,
    ) -> Self {
        let blue_assault = AssaultSummary::new(first_assailant, second_assailant);
        blue_assault.consequences().apply(
            first_assailant,
            second_assailant,
        );
        let red_assault = AssaultSummary::new(second_assailant, first_assailant);
        red_assault.consequences().apply(
            second_assailant,
            first_assailant,
        );
        Self {
            first_assailant_uuid: first_assailant.uuid().clone(),
            second_assailant_uuid: second_assailant.uuid().clone(),
            assaults: [blue_assault, red_assault],
            first_assailant_turn_end: first_assailant.end_turn(),
            second_assailant_turn_end: second_assailant.end_turn(),
        }
    }
}