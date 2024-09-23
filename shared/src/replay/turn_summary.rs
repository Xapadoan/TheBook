use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assault::assailant::Assailant;
use crate::assault::assault_summary::AssaultSummary;
use crate::assault::end_turn_consequences::EndTurnConsequences;

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnSummary {
    blue_corner_uuid: Uuid,
    red_corner_uuid: Uuid,
    assaults: [AssaultSummary; 2],
    blue_turn_end: EndTurnConsequences,
    red_turn_end: EndTurnConsequences,
}

impl TurnSummary {
    pub fn assaults(&self) -> &[AssaultSummary; 2] {
        &self.assaults
    }

    pub fn blue_turn_end(&self) -> &EndTurnConsequences {
        &self.blue_turn_end
    }

    pub fn red_turn_end(&self) -> &EndTurnConsequences {
        &self.red_turn_end
    }

    // server only
    pub fn new(
        blue_corner: &mut dyn Assailant,
        // blue_corner_inventory: &mut Inventory,
        red_corner: &mut dyn Assailant,
        // red_corner_inventory: &mut Inventory,
    ) -> Self {
        let blue_assault = AssaultSummary::new(blue_corner, red_corner);
        blue_assault.consequences().apply(
            blue_corner,
            // blue_corner_inventory,
            red_corner,
            // red_corner_inventory,
        );
        let red_assault = AssaultSummary::new(red_corner, blue_corner);
        red_assault.consequences().apply(
            red_corner,
            // red_corner_inventory,
            blue_corner,
            // blue_corner_inventory,
        );
        Self {
            blue_corner_uuid: blue_corner.uuid().clone(),
            red_corner_uuid: red_corner.uuid().clone(),
            assaults: [blue_assault, red_assault],
            blue_turn_end: blue_corner.end_turn(),
            red_turn_end: red_corner.end_turn(),
        }
    }
}