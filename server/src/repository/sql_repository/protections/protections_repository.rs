use sqlx::{
    mysql::MySqlArguments,
    query::Query,
    MySql,
};
use uuid::Uuid;

use crate::repository::sql_repository::query_builder::QueryBuilder;

use super::{CreateProtectionSchema, ProtectionModel, UpdateProtectionSchema};

pub struct ProtectionsQueryBuilder;

impl QueryBuilder for ProtectionsQueryBuilder {
    type Model = ProtectionModel;
    type CreateSchema = CreateProtectionSchema;
    type UpdateSchema = UpdateProtectionSchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO protections (\
                uuid,\
                name,\
                kind,\
                rupture,\
                damage_reduction,\
                dexterity_stat_modifier,\
                courage_stat_modifier\
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            item.uuid.to_string(),
            item.name,
            item.kind.as_str(),
            item.rupture,
            item.damage_reduction,
            item.dexterity_stat_modifier,
            item.courage_stat_modifier,
        )
    }
    fn read_query(uuid: &Uuid) -> sqlx::query::Map<
            'static,
            MySql,
            impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
            MySqlArguments
        > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM protections WHERE uuid=?",
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
            "SELECT * FROM protections",
        )
    }
    fn update_query(uuid: &Uuid, item: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "UPDATE protections SET rupture = ? WHERE uuid = ?",
            item.rupture,
            uuid.to_string(),
        )
    }
    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "DELETE FROM protections WHERE uuid = ?",
            uuid.to_string(),
        )
    }
}
