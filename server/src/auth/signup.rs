use shared::player::Player;
use sqlx::MySqlPool;

use crate::player::PlayerAPIError;
use crate::repository::sql_repository::players::{CreatePlayerSchema, PlayersRepository};
use crate::repository::RepositoryCreate;

pub struct SignUp<'a> {
    db_pool: &'a MySqlPool,
    username: String,
}

impl<'a> SignUp<'a> {
    pub fn new(
        db_pool: &'a MySqlPool,
        username: &str,
    ) -> Self {
        Self {
            db_pool,
            username: username.to_string(),
        }
    }
    pub async fn gen_player(&self) -> Result<Player, PlayerAPIError> {
        let player_repository = PlayersRepository::new(&self.db_pool);
        let player_schema = CreatePlayerSchema::new(&self.username);
        let created_player = player_repository.create(&player_schema).await?;

        Ok(created_player)
    }
}
