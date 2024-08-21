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

#[derive(Debug, Serialize, Deserialize)]
pub struct FightReplaySummary {
    replay_uuid: Uuid,
    winner: Option<Uuid>,
    loser: Option<Uuid>,
    tie: Option<(Uuid, Uuid)>,
    blue_corner_uuid: Uuid,
    red_corner_uuid: Uuid,
}

impl FightReplaySummary {
    pub fn winner(&self) -> &Option<Uuid> {
        &self.winner
    }

    pub fn loser(&self) -> &Option<Uuid> {
        &self.loser
    }

    pub fn tie(&self) -> &Option<(Uuid, Uuid)> {
        &self.tie
    }

    pub fn replay_uuid(&self) -> &Uuid {
        &self.replay_uuid
    }

    pub fn blue_corner_uuid(&self) -> &Uuid {
        &self.blue_corner_uuid
    }

    pub fn red_corner_uuid(&self) -> &Uuid {
        &self.red_corner_uuid
    }

    // server only
    pub fn new(
        replay_uuid: Uuid,
        winner: Option<Uuid>,
        loser: Option<Uuid>,
        tie: Option<(Uuid, Uuid)>,
        blue_corner_uuid: Uuid,
        red_corner_uuid: Uuid,
    ) -> Self {
        Self {
            replay_uuid,
            winner,
            loser,
            tie,
            blue_corner_uuid,
            red_corner_uuid,
        }
    }
}
