use sqlx::{mysql::MySqlArguments, query::Query, MySql};
use uuid::Uuid;

pub trait QueryBuilder {
    type Model;
    type CreateSchema;
    type UpdateSchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments>;
    fn read_query(uuid: &Uuid) -> sqlx::query::Map<
        'static,
        MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
        MySqlArguments
    >;
    fn list_query() -> sqlx::query::Map<
        'static,
        MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
        MySqlArguments
    >;
    fn update_query(uuid: &Uuid, item: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments>;
    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments>;
}
