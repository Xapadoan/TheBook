use sqlx::{mysql::MySqlArguments, query::{Map, Query}, MySql, Pool};
use uuid::Uuid;

use crate::repository::{sql_repository::{warriors::{UpdateWarriorSchema, WarriorsQueryBuilder}, QueryBuilder}, RepositoryCreate, RepositoryError};

use super::{CreateTournamentWarriorSchema, TournamentWarriorModel};

pub struct TournamentsWarriorsQueryBuilder;
impl TournamentsWarriorsQueryBuilder {
    fn search_one_query(
        tournament_uuid: &Uuid,
        player_uuid: &Uuid,
        warrior_uuid: &Uuid,
    ) -> Map<
        'static,
        MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<TournamentWarriorModel, sqlx::Error>,
        MySqlArguments
    > {
        sqlx::query_as!(
            TournamentWarriorModel,
            "SELECT * FROM tournaments_warriors WHERE \
                tournament_uuid = ? \
                AND player_uuid = ? \
                AND warrior_uuid = ? \
            LIMIT 1",
            tournament_uuid.to_string(),
            player_uuid.to_string(),
            warrior_uuid.to_string(),
        )
    }
}
impl QueryBuilder for TournamentsWarriorsQueryBuilder {
    type Model = TournamentWarriorModel;
    type CreateSchema = CreateTournamentWarriorSchema;
    type UpdateSchema = ();

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO tournaments_warriors (\
                uuid,\
                tournament_uuid,\
                player_uuid,\
                warrior_uuid\
            ) VALUES (?, ?, ?, ?)",
            item.uuid.to_string(),
            item.tournament_uuid.to_string(),
            item.player_uuid.to_string(),
            item.warrior_uuid.to_string(),
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
            "SELECT * FROM tournaments_warriors WHERE uuid = ? LIMIT 1",
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
            "SELECT * FROM tournaments_warriors"
        )
    }
    fn update_query(_: &Uuid, _: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        panic!("This is for trait compatibility, you should never update this data")
    }
    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!("DELETE FROM tournaments_warriors WHERE uuid = ?", uuid)
    }
}

pub struct TournamentsWarriorsRepository<'a> {
    db_pool: &'a Pool<MySql>,
}
impl<'a> TournamentsWarriorsRepository<'a> {
    pub fn new(db_pool: &'a Pool<MySql>) -> Self {
        Self { db_pool }
    }
}
impl<'a> RepositoryCreate<TournamentWarriorModel, CreateTournamentWarriorSchema> for TournamentsWarriorsRepository<'a> {
    async fn create(&self, item: &CreateTournamentWarriorSchema) -> Result<TournamentWarriorModel, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;

        let existing_entry = TournamentsWarriorsQueryBuilder::search_one_query(
            &item.tournament_uuid,
            &item.player_uuid,
            &item.warrior_uuid,
        )
            .fetch_optional(&mut *trx)
            .await?;

        let returned_entry = match existing_entry {
            Some(entry) => entry,
            None => {
                let warrior_model = WarriorsQueryBuilder::read_query(&item.warrior_uuid)
                    .fetch_one(&mut *trx)
                    .await?;
                let warrior_update_schema = UpdateWarriorSchema::new(
                    warrior_model.current_health,
                    warrior_model.max_health,
                    Some(item.tournament_uuid.to_string()),
                    warrior_model.nat_attack,
                    warrior_model.nat_parry,
                    warrior_model.nat_courage,
                    warrior_model.nat_dexterity,
                    warrior_model.nat_strength,
                    warrior_model.last_passive_heal,
                    warrior_model.experience,
                    warrior_model.level,
                    warrior_model.weapon_uuid,
                );
                WarriorsQueryBuilder::update_query(&item.warrior_uuid, &warrior_update_schema)
                    .execute(&mut *trx)
                    .await?;
                TournamentsWarriorsQueryBuilder::create_query(item)
                    .execute(&mut *trx)
                    .await?;

                let created_entry = TournamentsWarriorsQueryBuilder::read_query(&item.uuid)
                    .fetch_one(&mut *trx)
                    .await?;

                created_entry
            }
        };

        trx.commit().await?;
        Ok(returned_entry)
    }
}
