use std::collections::HashMap;

use shared::tournament::Tournament;
use sqlx::{mysql::MySqlArguments, query::{Map, Query}, MySql, Pool};
use uuid::Uuid;

use crate::repository::{sql_repository::QueryBuilder, RepositoryCreate, RepositoryError, RepositoryList, RepositoryRead, RepositoryUpdate};

use super::{tournament_model::TournamentWarriorModel, CreateTournamentSchema, TournamentFinalModel, TournamentModel, UpdateTournamentSchema};

pub struct TournamentsQueryBuilder;
impl QueryBuilder for TournamentsQueryBuilder {
    type Model = TournamentModel;
    type CreateSchema = CreateTournamentSchema;
    type UpdateSchema = UpdateTournamentSchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO tournaments (uuid, name, max_contestants) VALUES (?, ?, ?)",
            item.uuid.to_string(),
            item.name,
            item.max_contestants
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
            "SELECT * FROM tournaments WHERE uuid = ?",
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
            "SELECT * FROM tournaments",
        )
    }
    fn update_query(uuid: &Uuid, item: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "UPDATE tournaments SET started_at = ? WHERE uuid = ?",
            item.started_at,
            uuid.to_string(),
        )
    }
    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "DELETE FROM tournaments WHERE uuid = ?",
            uuid.to_string(),
        )
    }
}
impl TournamentsQueryBuilder {
    fn list_contestants_query(tournament_uuid: &Uuid) -> Map<
        'static,
        MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<TournamentWarriorModel, sqlx::Error>,
        MySqlArguments
    > {
        sqlx::query_as!(
            TournamentWarriorModel,
            "SELECT * FROM tournaments_warriors WHERE tournament_uuid = ?",
            tournament_uuid.to_string()
        )
    }
}

pub struct TournamentsRepository<'a> {
    db_pool: &'a Pool<MySql>
}
impl<'a> TournamentsRepository<'a> {
    pub fn new(db_pool: &'a Pool<MySql>) -> Self {
        Self { db_pool }
    }

    pub async fn list_mut(&self) -> Result<Vec<Tournament>, RepositoryError> {
        let tournaments = TournamentsQueryBuilder::list_query()
            .fetch_all(self.db_pool)
            .await?;

        let mut result = vec![];
        for tournament in tournaments {
            let tournament_uuid = Uuid::parse_str(&tournament.uuid);
            if let Err(_) = tournament_uuid {
                eprintln!("[WARN] Found tournament with invalid uuid: {}", tournament.uuid);
                continue;
            }
            let tournament_uuid = tournament_uuid?;
            let contestants = self.list_contestants(&tournament_uuid).await?;
            let tournament = TournamentFinalModel::new(tournament, contestants);
            let tournament = Tournament::try_from(tournament)?;

            result.push(tournament);
        }

        Ok(result)
    }

    async fn list_contestants(&self, tournament_uuid: &Uuid) -> Result<HashMap<Uuid, Vec<Uuid>>, RepositoryError> {
        let contestants = TournamentsQueryBuilder::list_contestants_query(tournament_uuid)
            .fetch_all(self.db_pool)
            .await?;

        let mut map = HashMap::new();
        for contestant in contestants {
            let player_uuid = Uuid::parse_str(&contestant.player_uuid);
            if let Err (e) = player_uuid {
                return Err(RepositoryError::from(e))
            }
            let player_uuid = player_uuid?;

            let warrior_uuid = Uuid::parse_str(&contestant.warrior_uuid);
            if let Err (e) = warrior_uuid {
                return Err(RepositoryError::from(e))
            }
            let warrior_uuid = warrior_uuid?;

            if !map.contains_key(&player_uuid) {
                map.insert(player_uuid, vec![warrior_uuid]);
            } else {
                map.get_mut(&player_uuid).unwrap().push(warrior_uuid);
            }
        }

        Ok(map)
    }
}
impl<'a> RepositoryCreate<Tournament, <TournamentsQueryBuilder as QueryBuilder>::CreateSchema> for TournamentsRepository<'a> {
    async fn create(&self, item: &<TournamentsQueryBuilder as QueryBuilder>::CreateSchema) -> Result<Tournament, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;
        TournamentsQueryBuilder::create_query(item)
            .execute(&mut *trx)
            .await?;
        
        let created_tournament = TournamentsQueryBuilder::read_query(&item.uuid)
            .fetch_one(&mut *trx)
            .await?;

        trx.commit().await?;
        let created_tournament = TournamentFinalModel::new(created_tournament, HashMap::new());
        let created_tournament = Tournament::try_from(created_tournament)?;
        Ok(created_tournament)
    }
}
impl<'a> RepositoryRead<Tournament> for TournamentsRepository<'a> {
    async fn read(&self, uuid: &Uuid) -> Result<Tournament, RepositoryError> {
        let tournament = TournamentsQueryBuilder::read_query(uuid)
            .fetch_one(self.db_pool)
            .await?;
        
        let tournament_uuid = Uuid::parse_str(&tournament.uuid);
        if let Err(e) = tournament_uuid {
            eprintln!("[ERROR] Found tournament with invalid uuid: {}", tournament.uuid);
            return Err(RepositoryError::from(e));
        }
        let tournament_uuid = tournament_uuid?;
        let contestants = self.list_contestants(&tournament_uuid).await?;

        let tournament = TournamentFinalModel::new(tournament, contestants);
        let tournament = Tournament::try_from(tournament)?;

        Ok(tournament)
    }
}
impl<'a> RepositoryList<Tournament> for TournamentsRepository<'a> {
    async fn list(&self) -> Result<Vec<Tournament>, RepositoryError> {
        let tournaments = TournamentsQueryBuilder::list_query()
            .fetch_all(self.db_pool)
            .await?;

        let mut result = vec![];
        for tournament in tournaments {
            let tournament_uuid = Uuid::parse_str(&tournament.uuid);
            if let Err(_) = tournament_uuid {
                eprintln!("[WARN] Found tournament with invalid uuid: {}", tournament.uuid);
                continue;
            }
            let tournament_uuid = tournament_uuid?;
            let contestants = self.list_contestants(&tournament_uuid).await?;
            let tournament = TournamentFinalModel::new(tournament, contestants);
            let tournament = Tournament::try_from(tournament)?;

            result.push(tournament);
        }

        Ok(result)
    }
}
impl<'a> RepositoryUpdate<Tournament, UpdateTournamentSchema> for TournamentsRepository<'a> {
    async fn update(&self, uuid: &Uuid, item: &UpdateTournamentSchema) -> Result<Tournament, RepositoryError> {
        TournamentsQueryBuilder::update_query(uuid, item)
            .execute(self.db_pool)
            .await?;
        let updated_tournament = self.read(uuid).await?;

        Ok(updated_tournament)
    }
    
}