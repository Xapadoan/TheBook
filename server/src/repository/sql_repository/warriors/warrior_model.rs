use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{
    equipment::weapon::Weapon,
    health::Health,
    stats::StatsManager,
    warrior::{
        body::{body_part::BodyPart, Body},
        Warrior,
    },
};
use sqlx::FromRow;
use uuid::Uuid;

use crate::repository::sql_repository::{body_parts::BodyPartFinalModel, weapons::WeaponModel};

#[derive(Debug, Serialize, Deserialize,  FromRow)]
pub struct WarriorModel {
    pub uuid: String,
    pub player_uuid: String,
    pub name: String,
    pub current_health: u8,
    pub max_health: u8,
    pub tournament_uuid: Option<String>,
    pub nat_attack: u8,
    pub nat_parry: u8,
    pub nat_courage: u8,
    pub nat_dexterity: u8,
    pub nat_strength: u8,
    pub last_passive_heal: DateTime<Utc>,
    pub experience: u64,
    pub level: u8,
    pub weapon_uuid: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

pub struct WarriorFinalModel {
    pub warrior_model: WarriorModel,
    pub weapon_model: Option<WeaponModel>,
    pub body_parts: Vec<BodyPartFinalModel>,
}
impl WarriorFinalModel {
    pub fn new(
        warrior_model: WarriorModel,
        weapon_model: Option<WeaponModel>,
        body_parts: Vec<BodyPartFinalModel>,
    ) -> Self {
        Self {
            warrior_model,
            weapon_model,
            body_parts,
        }
    }
}
impl TryFrom<WarriorFinalModel> for Warrior {
    type Error = uuid::Error;

    fn try_from(value: WarriorFinalModel) -> Result<Self, Self::Error> {
        let mut weapon = None;
        if let Some(weapon_model) = value.weapon_model {
            weapon = Some(Weapon::try_from(weapon_model)?);
        }
        let mut current_tournament = None;
        if let Some(tournament_uuid) = value.warrior_model.tournament_uuid {
            current_tournament = Some(Uuid::parse_str(&tournament_uuid)?);
        }
        let body_parts: Result<Vec<BodyPart>, uuid::Error> = value.body_parts
            .into_iter()
            .map(|model| { BodyPart::try_from(model) })
            .collect();
        let warrior = Self::new(
            Uuid::parse_str(&value.warrior_model.uuid)?,
            value.warrior_model.name,
            Health::new(
                value.warrior_model.max_health,
                value.warrior_model.current_health,
            ),
            weapon,
            current_tournament,
            Body::from_body_parts(body_parts?),
            vec![],
            StatsManager::new(
                value.warrior_model.nat_attack,
                value.warrior_model.nat_parry,
                value.warrior_model.nat_strength,
                value.warrior_model.nat_dexterity,
                value.warrior_model.nat_courage,
            ),
            false,
            value.warrior_model.last_passive_heal.timestamp(),
            value.warrior_model.experience,
            value.warrior_model.level,
        );

        Ok(warrior)
    }
}
