use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::tournament::Tournament;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug)]
pub struct TournamentWarriorModel {
    pub uuid: String,
    pub player_uuid: String,
    pub tournament_uuid: String,
    pub warrior_uuid: String,
}

#[derive(Debug, Serialize, Deserialize,  FromRow)]
pub struct TournamentModel {
    pub uuid: String,
    pub name: String,
    pub max_contestants: u8,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct TournamentFinalModel {
    tournament_model: TournamentModel,
    contestants: HashMap<Uuid, Vec<Uuid>>,
}
impl TournamentFinalModel {
    pub fn new(
        tournament_model: TournamentModel,
        contestants: HashMap<Uuid, Vec<Uuid>>,
    ) -> Self {
        Self {
            tournament_model,
            contestants,
        }
    }
}

impl TryFrom<TournamentFinalModel> for Tournament {
    type Error = uuid::Error;
    fn try_from(value: TournamentFinalModel) -> Result<Self, Self::Error> {
        let tournament = Tournament::new(
            Uuid::parse_str(&value.tournament_model.uuid)?,
            value.tournament_model.name,
            value.tournament_model.max_contestants as usize,
            value.contestants,
            HashMap::new(),
        );
        Ok(tournament)
    }
}
