use shared::{
    equipment::weapon::Weapon,
    player::Player,
    random::Random, 
    warrior::Warrior,
};
use sqlx::{mysql::MySqlArguments, query::{Map, Query}, MySql, Pool};
use uuid::Uuid;

use crate::repository::{sql_repository::{body_parts::{BodyPartFinalModel, BodyPartsQueryBuilder}, query_builder::QueryBuilder, warriors::{CreateWarriorSchema, WarriorFinalModel, WarriorsQueryBuilder}, weapons::{CreateWeaponSchema, WeaponsQueryBuilder}}, RepositoryCreate, RepositoryDelete, RepositoryError, RepositoryRead};

use super::{CreatePlayerSchema, PlayerFinalModel, PlayerModel, UpdatePlayerSchema};

pub struct PlayersQueryBuilder;
impl QueryBuilder for PlayersQueryBuilder {
    type Model = PlayerModel;
    type CreateSchema = CreatePlayerSchema;
    type UpdateSchema = UpdatePlayerSchema;
    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO players (uuid, username) VALUES (?, ?)",
            item.uuid.to_string(),
            item.username,
        )
    }
    fn read_query(uuid: &Uuid) -> Map<
            'static,
            MySql,
            impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
            MySqlArguments
        > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM players WHERE uuid = ?",
            uuid.to_string(),
        )
    }

    fn list_query() -> sqlx::query::Map<
            'static,
            MySql,
            impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
            MySqlArguments
        > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM players",
        )
    }
    fn update_query(_: &Uuid, _: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        panic!("Cannot update player");
    }
    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "DELETE FROM players WHERE uuid = ?",
            uuid.to_string(),
        )
    }
}

pub struct PlayersRepository<'a> {
    db_pool: &'a Pool<MySql>,
}
impl<'a> PlayersRepository<'a> {
    pub fn new(db_pool: &'a Pool<MySql>) -> Self {
        Self { db_pool }
    }
}
impl<'a> RepositoryCreate<Player, CreatePlayerSchema> for PlayersRepository<'a> {
    async fn create(&self, item: &CreatePlayerSchema) -> Result<Player, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;
        // let player_schema = CreatePlayerSchema::new(&item.username);
        PlayersQueryBuilder::create_query(&item)
            .execute(&mut *trx)
            .await?;

        let mut i = 0;
        while i < 8 {
            let weapon_schema = CreateWeaponSchema::from(Weapon::random());
            WeaponsQueryBuilder::create_query(&weapon_schema)
                .execute(&mut *trx)
                .await?;
            let warrior_schema = CreateWarriorSchema::new(
                Warrior::random(),
                &item.uuid,
                &weapon_schema.uuid,
            );
            WarriorsQueryBuilder::create_query(&warrior_schema)
                .execute(&mut *trx)
                .await?;
            BodyPartsQueryBuilder::create_entire_body_query(&warrior_schema.uuid)
                .execute(&mut *trx)
                .await?;
            i += 1;
        }
        trx.commit().await?;

        let created_player = self.read(&item.uuid).await?;
        Ok(created_player)
    }
}
impl<'a> RepositoryRead<Player> for PlayersRepository<'a> {
    async fn read(&self, uuid: &Uuid) -> Result<Player, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;
        let player = PlayersQueryBuilder::read_query(uuid)
            .fetch_one(&mut *trx)
            .await?;
        let warriors = WarriorsQueryBuilder::read_player_warriors(uuid)
            .fetch_all(&mut *trx)
            .await?;
        let mut warriors_models = vec![];
        for warrior in warriors.into_iter() {
            let weapon_uuid = Uuid::parse_str(warrior.weapon_uuid.as_ref().unwrap())?;
            let weapon = WeaponsQueryBuilder::read_query(&weapon_uuid)
                .fetch_one(&mut *trx)
                .await?;

            let warrior_uuid = Uuid::parse_str(&warrior.uuid)?;
            let body_parts = BodyPartsQueryBuilder::read_entire_body_query(&warrior_uuid)
                .fetch_all(&mut *trx)
                .await?;
            let body_parts = body_parts
                .into_iter()
                .map(|part| { BodyPartFinalModel::new(part, None) })
                .collect();
            let warrior = WarriorFinalModel::new(
                warrior,
                Some(weapon),
                body_parts,
            );
            warriors_models.push(warrior);
        }

        trx.commit().await?;

        let player = PlayerFinalModel::new(
            player,
            warriors_models,
        );
        let player = Player::try_from(player)?;
        Ok(player)
    }
}
impl<'a> RepositoryDelete for PlayersRepository<'a> {
    async fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError> {
        PlayersQueryBuilder::delete_query(uuid)
            .execute(self.db_pool)
            .await?;

        Ok(())
    }
}
