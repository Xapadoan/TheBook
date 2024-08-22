use std::error::Error;
use std::fmt::Display;

use shared::player::Player;
use uuid::Uuid;

use crate::repository::{PlayerRepository, Repository, RepositoryError};

pub fn login_from_session(uuid: &Uuid) -> Result<Player, SessionError> {
    let repo = PlayerRepository::build()?;
    let player = repo.get_by_uuid(uuid)?;
    Ok(player)
}

#[derive(Debug)]
pub struct SessionError {
    message: String,
}

impl SessionError {
    fn new(message: String) -> Self {
        Self { message: format!("Session Error\n{message}") }
    }
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SessionError {}

impl From<RepositoryError> for SessionError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
    }
}
