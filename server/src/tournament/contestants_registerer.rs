use std::{error::Error, fmt::Display};

use shared::{tournament::{contestant::TournamentContestant, Tournament, TournamentError}, unique_entity::UniqueEntity, warrior::Warrior};
use uuid::Uuid;

use crate::repository::{sql_repository::tournaments_warriors::{CreateTournamentWarriorSchema, TournamentWarriorModel}, RepositoryCreate, RepositoryError};

pub struct ContestantsRegisterer<T>
where
    T: RepositoryCreate<TournamentWarriorModel, CreateTournamentWarriorSchema>
{
    repo: T,
}

impl<T> ContestantsRegisterer<T>
where
    T: RepositoryCreate<TournamentWarriorModel, CreateTournamentWarriorSchema>
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
    pub async fn register_contestant(
        &self,
        player_uuid: &Uuid,
        tournament: &mut Tournament,
        warrior: &mut Warrior,
    ) -> Result<(), ContestantsRegistererError> {
        tournament.add_contestant(player_uuid, warrior)?;
        // Insert player tournament if needed
        let create_schema = CreateTournamentWarriorSchema::new(
            tournament.uuid().clone(),
            player_uuid.clone(),
            warrior.uuid().clone(),
        );
        let registration = self.repo.create(&create_schema).await?;
        eprintln!("[DEBUG] Registered warrior {} to tournament {}", registration.warrior_uuid, registration.tournament_uuid);
        warrior.set_current_tournament(Some(tournament.uuid().clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub struct ContestantsRegistererError {
    message: String,
}

impl ContestantsRegistererError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Contestants Registerer Error:\n{message}") }
    }
}

impl Display for ContestantsRegistererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ContestantsRegistererError {}

impl From<TournamentError> for ContestantsRegistererError {
    fn from(value: TournamentError) -> Self {
        Self::new(&format!("Tournament Error:\n{value}"))
    }
}

impl From<sqlx::Error> for ContestantsRegistererError {
    fn from(value: sqlx::Error) -> Self {
        Self::new(&format!("SQLX Error:\n{value}"))
    }
}

impl From<RepositoryError> for ContestantsRegistererError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}
