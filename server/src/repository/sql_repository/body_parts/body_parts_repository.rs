use shared::warrior::body::body_part::ALL_BODY_PARTS;
use sqlx::{mysql::{MySqlArguments, MySqlRow}, query::{Map, Query}, MySql, Transaction};
use uuid::Uuid;

use crate::repository::{sql_repository::query_builder::QueryBuilder, RepositoryError};

use super::{BodyPartModel, CreateBodyPartSchema, UpdateBodyPartSchema};

pub struct BodyPartsQueryBuilder {}
impl BodyPartsQueryBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create_entire_body_query(warrior_uuid: &Uuid) -> Query<'_, MySql, MySqlArguments> {
        let schemas = ALL_BODY_PARTS.map(
            |kind| {
                CreateBodyPartSchema::new(&kind, warrior_uuid)
            }
        );
        sqlx::query!(
            "INSERT INTO body_parts \
                (uuid, warrior_uuid, kind) \
            VALUES \
            (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?),\
            (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?),\
            (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?),\
            (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?), (?, ?, ?),\
            (?, ?, ?), (?, ?, ?), (?, ?, ?)",
            schemas[0].uuid.to_string(), schemas[0].warrior_uuid.to_string(), schemas[0].kind.to_string(),
            schemas[1].uuid.to_string(), schemas[1].warrior_uuid.to_string(), schemas[1].kind.to_string(),
            schemas[2].uuid.to_string(), schemas[2].warrior_uuid.to_string(), schemas[2].kind.to_string(),
            schemas[3].uuid.to_string(), schemas[3].warrior_uuid.to_string(), schemas[3].kind.to_string(),
            schemas[4].uuid.to_string(), schemas[4].warrior_uuid.to_string(), schemas[4].kind.to_string(),
            schemas[5].uuid.to_string(), schemas[5].warrior_uuid.to_string(), schemas[5].kind.to_string(),
            schemas[6].uuid.to_string(), schemas[6].warrior_uuid.to_string(), schemas[6].kind.to_string(),
            schemas[7].uuid.to_string(), schemas[7].warrior_uuid.to_string(), schemas[7].kind.to_string(),
            schemas[8].uuid.to_string(), schemas[8].warrior_uuid.to_string(), schemas[8].kind.to_string(),
            schemas[9].uuid.to_string(), schemas[9].warrior_uuid.to_string(), schemas[9].kind.to_string(),

            schemas[10].uuid.to_string(), schemas[10].warrior_uuid.to_string(), schemas[10].kind.to_string(),
            schemas[11].uuid.to_string(), schemas[11].warrior_uuid.to_string(), schemas[11].kind.to_string(),
            schemas[12].uuid.to_string(), schemas[12].warrior_uuid.to_string(), schemas[12].kind.to_string(),
            schemas[13].uuid.to_string(), schemas[13].warrior_uuid.to_string(), schemas[13].kind.to_string(),
            schemas[14].uuid.to_string(), schemas[14].warrior_uuid.to_string(), schemas[14].kind.to_string(),
            schemas[15].uuid.to_string(), schemas[15].warrior_uuid.to_string(), schemas[15].kind.to_string(),
            schemas[16].uuid.to_string(), schemas[16].warrior_uuid.to_string(), schemas[16].kind.to_string(),
            schemas[17].uuid.to_string(), schemas[17].warrior_uuid.to_string(), schemas[17].kind.to_string(),
            schemas[18].uuid.to_string(), schemas[18].warrior_uuid.to_string(), schemas[18].kind.to_string(),
            schemas[19].uuid.to_string(), schemas[19].warrior_uuid.to_string(), schemas[19].kind.to_string(),

            schemas[20].uuid.to_string(), schemas[20].warrior_uuid.to_string(), schemas[20].kind.to_string(),
            schemas[21].uuid.to_string(), schemas[21].warrior_uuid.to_string(), schemas[21].kind.to_string(),
            schemas[22].uuid.to_string(), schemas[22].warrior_uuid.to_string(), schemas[22].kind.to_string(),
        )
    }

    pub fn read_entire_body_query(warrior_uuid: &Uuid) -> Map<
            'static,
            MySql,
            impl FnMut(MySqlRow) -> Result<BodyPartModel, sqlx::Error>,
            MySqlArguments
        > {
        sqlx::query_as!(
            BodyPartModel,
            "SELECT * FROM body_parts WHERE warrior_uuid = ?",
            warrior_uuid.to_string(),
        )
    }
}
impl QueryBuilder for BodyPartsQueryBuilder {
    type Model = BodyPartModel;
    type CreateSchema = CreateBodyPartSchema;
    type UpdateSchema = UpdateBodyPartSchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO body_parts (\
                uuid,\
                warrior_uuid,\
                kind\
            ) VALUES (?, ?, ?)",
            item.uuid.to_string(),
            item.warrior_uuid.to_string(),
            item.kind.to_string(),
        )
    }

    fn read_query(uuid: &Uuid) -> Map<
            'static,
            MySql,
            impl FnMut(MySqlRow) -> Result<Self::Model, sqlx::Error>,
            MySqlArguments
        > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM body_parts WHERE uuid=?",
            uuid.to_string(),
        )
    }

    fn list_query() -> Map<
            'static,
            MySql,
            impl FnMut(MySqlRow) -> Result<Self::Model, sqlx::Error>,
            MySqlArguments
        > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM body_parts",
        )
    }

    fn update_query(uuid: &Uuid, item: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "UPDATE body_parts SET is_broken = ?, protection_uuid = ? WHERE uuid = ?",
            item.is_broken,
            item.protection_uuid,
            uuid.to_string()
        )
    }

    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "DELETE FROM body_parts WHERE uuid = ?",
            uuid.to_string(),
        )
    }
}

pub struct BodyPartsRepository {
    query_builder: BodyPartsQueryBuilder,
}
impl BodyPartsRepository {
    pub fn new() -> Self {
        Self { query_builder: BodyPartsQueryBuilder {} }
    }
    pub async fn create_entire_body<'a>(&self, warrior_uuid: &Uuid, mut trx: Transaction<'a, MySql>) -> Result<(), RepositoryError> {
        BodyPartsQueryBuilder::create_entire_body_query(warrior_uuid)
            .execute(&mut *trx)
            .await?;

        Ok(())
    }
}
// impl TrxRepository for BodyPartsRepository {
//     type Model = BodyPartModel;
//     type CreateSchema = CreateBodyPartSchema;
//     type UpdateSchema = UpdateBodyPartSchema;

//     async fn create_trx<'a>(&self, item: &Self::CreateSchema, mut trx: Transaction<'a, MySql>) -> Result<Self::Model, RepositoryError> {
//         self.query_builder
//             .create_query(item)
//             .execute(&mut *trx)
//             .await?;

//         let created = self.read_trx(&item.uuid, trx).await?;
//         Ok(created)
//     }
//     async fn read_trx<'a>(&self, uuid: &Uuid, mut trx: Transaction<'a, MySql>) -> Result<Self::Model, RepositoryError> {
//         let res = self.query_builder
//             .read_query(uuid)
//             .fetch_one(&mut *trx)
//             .await?;

//         Ok(res)
//     }
//     async fn list_trx<'a>(&self, mut trx: Transaction<'a, MySql>) -> Result<Vec<Self::Model>, RepositoryError> {
//         let res = self.query_builder
//             .list_query()
//             .fetch_all(&mut *trx)
//             .await?;

//         Ok(res)
//     }
//     async fn update_trx<'a>(&mut self, uuid: &Uuid, item: &Self::UpdateSchema, mut trx: Transaction<'a, MySql>) -> Result<Self::Model, RepositoryError> {
//         self.query_builder
//             .update_query(uuid, item)
//             .execute(&mut *trx)
//             .await?;

//         let updated = self.read_trx(uuid, trx).await?;
//         Ok(updated)
//     }
//     async fn delete_trx<'a>(&mut self, uuid: &Uuid, mut trx: Transaction<'a, MySql>) -> Result<(), RepositoryError> {
//         self.query_builder
//             .delete_query(uuid)
//             .execute(&mut *trx)
//             .await?;

//         Ok(())
//     }
// }