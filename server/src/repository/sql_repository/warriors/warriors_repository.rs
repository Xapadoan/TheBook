use shared::{equipment::weapon::OptionalMutableWeapon, random::Random, unique_entity::UniqueEntity, warrior::Warrior};
use sqlx::{mysql::MySqlArguments, query::{Map, Query}, MySql, Pool};
use uuid::Uuid;

use crate::repository::{
    sql_repository::{
        body_parts::{BodyPartFinalModel, BodyPartsQueryBuilder},
        query_builder::QueryBuilder,
        weapons::{CreateWeaponSchema, WeaponsQueryBuilder},
    },
    RepositoryCreate,
    RepositoryError,
    RepositoryRead,
    RepositoryUpdate,
};

use super::{CreateWarriorSchema, UpdateWarriorSchema, WarriorFinalModel, WarriorModel};

pub struct WarriorsQueryBuilder;
impl WarriorsQueryBuilder {
    pub fn read_player_warriors(player_uuid: &Uuid) -> Map<
            'static,
            MySql,
            impl FnMut(sqlx::mysql::MySqlRow) -> Result<WarriorModel, sqlx::Error>,
            MySqlArguments
        > {
            sqlx::query_as!(
                WarriorModel,
                "SELECT * FROM warriors WHERE player_uuid = ?",
                player_uuid.to_string(),
            )
        }
}
impl QueryBuilder for WarriorsQueryBuilder {
    type Model = WarriorModel;
    type CreateSchema = CreateWarriorSchema;
    type UpdateSchema = UpdateWarriorSchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO warriors (\
                uuid,\
                player_uuid,\
                name,\
                current_health,\
                max_health,\
                nat_attack,\
                nat_parry,\
                nat_courage,\
                nat_dexterity,\
                nat_strength,\
                weapon_uuid\
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            item.uuid.to_string(),
            item.player_uuid.to_string(),
            item.name,
            item.health,
            item.health,
            item.nat_attack,
            item.nat_parry,
            item.nat_courage,
            item.nat_dexterity,
            item.nat_strength,
            item.weapon_uuid,
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
            "SELECT * FROM warriors WHERE uuid = ?",
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
            "SELECT * FROM warriors",
        )
    }

    fn update_query(uuid: &Uuid, item: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "UPDATE warriors SET \
                current_health = ?,\
                max_health = ?,\
                tournament_uuid = ?,\
                nat_attack = ?,\
                nat_parry = ?,\
                nat_courage = ?,\
                nat_dexterity = ?,\
                nat_strength = ?,\
                last_passive_heal = ?,\
                experience = ?,\
                level = ?,\
                weapon_uuid = ? \
            WHERE uuid = ?",
            item.current_health,
            item.max_health,
            item.tournament_uuid,
            item.nat_attack,
            item.nat_parry,
            item.nat_courage,
            item.nat_dexterity,
            item.nat_strength,
            item.last_passive_heal,
            item.experience,
            item.level,
            item.weapon_uuid,
            uuid.to_string(),
        )
    }

    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "DELETE FROM warriors WHERE uuid = ?",
            uuid.to_string()
        )
    }
}

pub struct WarriorsRepository<'a> {
    db_pool: &'a Pool<MySql>,
}
impl<'a> WarriorsRepository<'a> {
    pub fn new(db_pool: &'a Pool<MySql>) -> Self {
        Self { db_pool }
    }
    // Should this be here ?
    // Or on a higher level of abstraction ?
    pub async fn create_random(&self, player_uuid: &Uuid) -> Result<Warrior, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;

        let mut warrior = Warrior::random();
        let weapon = warrior.weapon_mut().take();
        let warrior_schema = CreateWarriorSchema::new(
            warrior,
            player_uuid,
            weapon.as_ref().unwrap().uuid(),
        );
        let weapon_schema = CreateWeaponSchema::from(weapon.unwrap());

        WeaponsQueryBuilder::create_query(&weapon_schema)
            .execute(&mut *trx)
            .await?;
        BodyPartsQueryBuilder::create_entire_body_query(&warrior_schema.uuid)
            .execute(&mut *trx)
            .await?;
        WarriorsQueryBuilder::create_query(&warrior_schema)
            .execute(&mut *trx)
            .await?;
        trx.commit().await?;

        let warrior = self.read(&warrior_schema.uuid).await?;
        Ok(warrior)
    }
}

impl<'a> RepositoryCreate<Warrior, CreateWarriorSchema> for WarriorsRepository<'a> {
    async fn create(&self, item: &CreateWarriorSchema) -> Result<Warrior, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;
        WarriorsQueryBuilder::create_query(item)
            .execute(&mut *trx)
            .await?;
        BodyPartsQueryBuilder::create_entire_body_query(&item.uuid)
            .execute(&mut *trx)
            .await?;
        trx.commit().await?;
        let created_warrior = self.read(&item.uuid).await?;

        Ok(created_warrior)
    }
}
impl<'a> RepositoryRead<Warrior> for WarriorsRepository<'a> {
    async fn read(&self, uuid: &Uuid) -> Result<Warrior, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;

        let warrior = WarriorsQueryBuilder::read_query(uuid)
            .fetch_one(&mut *trx)
            .await?;
        let body_parts = BodyPartsQueryBuilder::read_entire_body_query(&uuid)
            .fetch_all(&mut *trx)
            .await?;
        let body_parts = body_parts
            .into_iter()
            .map(|part| { BodyPartFinalModel::new(part, None) })
            .collect();
        let weapon = match &warrior.weapon_uuid {
            None => None,
            Some(weapon_uuid) => {
                let weapon_uuid = Uuid::parse_str(&weapon_uuid)?;
                let weapon = WeaponsQueryBuilder::read_query(&weapon_uuid)
                    .fetch_one(&mut *trx)
                    .await?;

                Some(weapon)
            }
        };

        trx.commit().await?;
        let warrior = WarriorFinalModel::new(
            warrior,
            weapon,
            body_parts,
        );
        let warrior = Warrior::try_from(warrior)?;

        Ok(warrior)
    }
}
impl<'a> RepositoryUpdate<Warrior, UpdateWarriorSchema> for WarriorsRepository<'a> {
    async fn update(&self, uuid: &Uuid, item: &UpdateWarriorSchema) -> Result<Warrior, RepositoryError> {
        WarriorsQueryBuilder::update_query(uuid, item)
            .execute(self.db_pool)
            .await?;

        let updated_warrior = self.read(uuid).await?;
        Ok(updated_warrior)
    }
}
