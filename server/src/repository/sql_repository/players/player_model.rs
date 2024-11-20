use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{inventory::Inventory, player::Player, warrior::Warrior};
use sqlx::FromRow;
use uuid::Uuid;

use crate::repository::sql_repository::warriors::WarriorFinalModel;

#[derive(Debug, Serialize, Deserialize,  FromRow)]
pub struct PlayerModel {
    pub uuid: String,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
pub struct PlayerFinalModel {
    pub player_model: PlayerModel,
    pub warriors_models: Vec<WarriorFinalModel>,
}
impl PlayerFinalModel {
    pub fn new(
        player_model: PlayerModel,
        warriors_models: Vec<WarriorFinalModel>,
    ) -> Self {
        Self {
            player_model,
            warriors_models,
        }
    }
}
impl TryFrom<PlayerFinalModel> for Player {
    type Error = uuid::Error;

    fn try_from(value: PlayerFinalModel) -> Result<Self, Self::Error> {
        let warriors: Result<Vec<Warrior>, uuid::Error> = value.warriors_models
            .into_iter()
            .map(|model| Warrior::try_from(model))
            .collect();
        let player = Self::new(
            Uuid::parse_str(&value.player_model.uuid)?,
            value.player_model.username.clone(),
            value.player_model.username,
            warriors?,
            Inventory::new(),
        );

        Ok(player)
    }
}
