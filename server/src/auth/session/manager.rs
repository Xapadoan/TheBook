use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use shared::auth::Session;
use shared::player::{Player, PlayerBuildError};
use shared::random::Random;
use shared::unique_entity::UniqueEntity;
use uuid::Uuid;

use crate::repository::{FileRepository, PlayerRepository, Repository, RepositoryError};

pub struct SessionManager {
    repo: FileRepository<SessionContents>
}

impl SessionManager {
    pub fn build() -> Result<Self, SessionManagerError> {
        let repo = FileRepository::build(PathBuf::from("sessions"))?;
        Ok(Self { repo })
    }

    pub fn read_player(&self, uuid: &Uuid) -> Result<Player, SessionManagerError> {
        let session = self.repo.get_by_uuid(uuid)?;
        let repo = PlayerRepository::build()?;
        let player = repo.get_by_uuid(&session.player_uuid)?;
        Ok(player)
    }

    pub fn create_session(&self, player_uuid: &Uuid) -> Result<Session, SessionManagerError> {
        let session = Session::random();
        let content = SessionContents { session_uuid: session.uuid().clone(), player_uuid: player_uuid.clone() };
        self.repo.create(&content)?;
        Ok(session)
    }


}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionContents {
    session_uuid: Uuid,
    player_uuid: Uuid,
}

impl UniqueEntity for SessionContents {
    fn uuid(&self) -> &Uuid {
        &self.session_uuid
    }
}

#[derive(Debug)]
pub struct SessionManagerError {
    message: String,
}

impl SessionManagerError {
    fn new(message: String) -> Self {
        Self { message: format!("Session Error\n{message}") }
    }
}

impl Display for SessionManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SessionManagerError {}

impl From<RepositoryError> for SessionManagerError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
    }
}

impl From<SessionManagerError> for PlayerBuildError {
    fn from(value: SessionManagerError) -> Self {
        eprintln!("[WARN] Should remove impl From<SessionManagerError> for PlayerBuildError");
        Self::new(format!("Session Error:\n{value}"))
    }
}
