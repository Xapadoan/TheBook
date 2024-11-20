use chrono::{DateTime, Utc};
use shared::{name::Name, tournament::Tournament, unique_entity::UniqueEntity};
use uuid::Uuid;

pub struct CreateTournamentSchema {
    pub uuid: Uuid,
    pub name: String,
    pub max_contestants: u8,
}
impl From<Tournament> for CreateTournamentSchema {
    fn from(value: Tournament) -> Self {
        Self {
            uuid: value.uuid().clone(),
            name: value.name().to_string(),
            max_contestants: value.max_contestants() as u8,
        }
    }
}

pub struct UpdateTournamentSchema {
    pub started_at: DateTime<Utc>,
}

pub struct CreateTournamentPlayerSchema {
    pub uuid: Uuid,
    pub tournament_uuid: Uuid,
    pub player_uuid: Uuid,
}
impl CreateTournamentPlayerSchema {
    pub fn new(tournament_uuid: Uuid, player_uuid: Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            tournament_uuid,
            player_uuid,
        }
    }
}
