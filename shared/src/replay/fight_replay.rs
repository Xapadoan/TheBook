use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::turn_summary::TurnSummary;

#[derive(Debug, Serialize, Deserialize)]
pub struct FightReplay {
    uuid: Uuid,
    blue_corner_uuid: Uuid,
    red_corner_uuid: Uuid,
    turn_summaries: Vec<TurnSummary>
}

impl FightReplay {
    // Should be used only by ReplayManager
    pub fn new(
        uuid: Uuid,
        blue_corner_uuid: Uuid,
        red_corner_uuid: Uuid,
        turn_summaries: Vec<TurnSummary>,
    ) -> Self {
        Self {
            uuid,
            blue_corner_uuid,
            red_corner_uuid,
            turn_summaries,
        }
    }

    pub fn blue_corner_uuid(&self) -> &Uuid {
        &self.blue_corner_uuid
    }

    pub fn red_corner_uuid(&self) -> &Uuid {
        &self.red_corner_uuid
    }

    pub fn turn_summaries(&self) -> &Vec<TurnSummary> {
        &self.turn_summaries
    }
}
