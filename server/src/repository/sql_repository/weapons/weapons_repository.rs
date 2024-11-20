use sqlx::{
    mysql::MySqlArguments,
    query::Query,
    MySql,
    Pool,
};
use uuid::Uuid;

use crate::repository::{main::{RepositoryDelete, RepositoryUpdate}, sql_repository::query_builder::QueryBuilder, Repository, RepositoryCreate, RepositoryError, RepositoryList, RepositoryRead};

use super::{weapon_model::WeaponModel, weapon_schemas::{CreateWeaponSchema, UpdateWeaponSchema}};

pub struct WeaponsQueryBuilder;

impl QueryBuilder for WeaponsQueryBuilder {
    type Model = WeaponModel;
    type CreateSchema = CreateWeaponSchema;
    type UpdateSchema = UpdateWeaponSchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO weapons (\
                uuid,\
                name,\
                kind,\
                is_sharp,\
                is_two_handed,\
                rupture,\
                additional_damages,\
                attack_stat_modifier,\
                parry_stat_modifier,\
                courage_stat_modifier\
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            item.uuid.to_string(),
            item.name,
            item.kind.as_str(),
            item.is_sharp,
            item.is_two_handed,
            item.rupture,
            item.additional_damages,
            item.attack_stat_modifier,
            item.parry_stat_modifier,
            item.courage_stat_modifier,
        )
    }
    fn read_query(uuid: &Uuid) -> sqlx::query::Map<
        'static,
        MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
        MySqlArguments,
    > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM weapons WHERE uuid=?",
            uuid.to_string(),
        )
    }
    fn list_query() -> sqlx::query::Map<
        'static,
        MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
        MySqlArguments,
    > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM weapons",
        )
    }
    fn update_query(uuid: &Uuid, item: &Self::UpdateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "UPDATE weapons SET rupture=? WHERE uuid=?",
            item.rupture,
            uuid.to_string(),
        )
    }
    fn delete_query(uuid: &Uuid) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "DELETE FROM weapons WHERE uuid=?",
            uuid.to_string(),
        )
    }
}

pub struct WeaponsPoolRepository<'a> {
    pool: &'a Pool<MySql>,
    query_builder: WeaponsQueryBuilder,
}

impl<'a> WeaponsPoolRepository<'a> {
    pub fn new(pool: &'a Pool<MySql>) -> Self {
        Self { pool, query_builder: WeaponsQueryBuilder {} }
    }
}

impl<'a> RepositoryCreate<
    <WeaponsQueryBuilder as QueryBuilder>::Model,
    <WeaponsQueryBuilder as QueryBuilder>::CreateSchema
> for WeaponsPoolRepository<'a> {
    async fn create(
        &self,
        item: &<WeaponsQueryBuilder as QueryBuilder>::CreateSchema,
    ) -> Result<<WeaponsQueryBuilder as QueryBuilder>::Model, RepositoryError> {
        WeaponsQueryBuilder::create_query(item)
            .execute(self.pool)
            .await?;
        let res = self.read(&item.uuid).await?;
        Ok(res)
    }
}
impl<'a> RepositoryRead<<WeaponsQueryBuilder as QueryBuilder>::Model> for WeaponsPoolRepository<'a> {
    async fn read(&self, uuid: &uuid::Uuid) -> Result<<WeaponsQueryBuilder as QueryBuilder>::Model, RepositoryError> {
        let res = WeaponsQueryBuilder::read_query(uuid)
            .fetch_one(self.pool)
            .await?;
        Ok(res)
    }
}
impl<'a> RepositoryList<<WeaponsQueryBuilder as QueryBuilder>::Model> for WeaponsPoolRepository<'a> {
    async fn list(&self) -> Result<Vec<<WeaponsQueryBuilder as QueryBuilder>::Model>, RepositoryError> {
        let res = WeaponsQueryBuilder::list_query()
            .fetch_all(self.pool)
            .await?;
        Ok(res)
    }
}
impl<'a> RepositoryUpdate<
    <WeaponsQueryBuilder as QueryBuilder>::Model,
    <WeaponsQueryBuilder as QueryBuilder>::UpdateSchema
> for WeaponsPoolRepository<'a> {
    async fn update(
        &self,
        uuid: &Uuid,
        item: &<WeaponsQueryBuilder as QueryBuilder>::UpdateSchema,
    ) -> Result<<WeaponsQueryBuilder as QueryBuilder>::Model, RepositoryError> {
        WeaponsQueryBuilder::update_query(uuid, item)
            .execute(self.pool)
            .await?;

        let res = self.read(uuid).await?;
        Ok(res)
    }
}
impl<'a> RepositoryDelete for WeaponsPoolRepository<'a> {
    async fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError> {
        WeaponsQueryBuilder::delete_query(uuid)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}

impl<'a> Repository for WeaponsPoolRepository<'a> {
    type Model = <WeaponsQueryBuilder as QueryBuilder>::Model;
    type CreateSchema = <WeaponsQueryBuilder as QueryBuilder>::CreateSchema;
    type UpdateSchema = <WeaponsQueryBuilder as QueryBuilder>::UpdateSchema;

    // async fn create(&self, item: &Self::CreateSchema) -> Result<Self::Model, RepositoryError> {
    //     WeaponsQueryBuilder::create_query(item)
    //         .execute(self.pool)
    //         .await?;
    //     let res = self.read(&item.uuid).await?;
    //     Ok(res)
    // }

    // async fn read(&self, uuid: &uuid::Uuid) -> Result<Self::Model, RepositoryError> {
    //     let res = WeaponsQueryBuilder::read_query(uuid)
    //         .fetch_one(self.pool)
    //         .await?;
    //     Ok(res)
    // }

    // async fn list(&self) -> Result<Vec<Self::Model>, RepositoryError> {
    //     let res = WeaponsQueryBuilder::list_query()
    //         .fetch_all(self.pool)
    //         .await?;
    //     Ok(res)
    // }

    // async fn update(&self, uuid: &Uuid, item: &Self::UpdateSchema) -> Result<Self::Model, RepositoryError> {
    //     WeaponsQueryBuilder::update_query(uuid, item)
    //         .execute(self.pool)
    //         .await?;

    //     let res = self.read(uuid).await?;
    //     Ok(res)
    // }

    // async fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError> {
    //     WeaponsQueryBuilder::delete_query(uuid)
    //         .execute(self.pool)
    //         .await?;
    //     Ok(())
    // }
}

// impl<'a> TrxRepository for WeaponsPoolRepository<'a> {
//     type Model = <WeaponsQueryBuilder as QueryBuilder>::Model;
//     type CreateSchema = <WeaponsQueryBuilder as QueryBuilder>::CreateSchema;
//     type UpdateSchema = <WeaponsQueryBuilder as QueryBuilder>::UpdateSchema;

//     async fn create_trx<'b>(&self, item: &Self::CreateSchema, mut trx: sqlx::Transaction<'b, MySql>) -> Result<Self::Model, RepositoryError> {
//         WeaponsQueryBuilder::create_query(item)
//             .execute(&mut *trx)
//             .await?;

//         let res = self.read_trx(&item.uuid, trx).await?;
//         Ok(res)
//     }

//     async fn read_trx<'b>(&self, uuid: &uuid::Uuid, mut trx: sqlx::Transaction<'b, MySql>) -> Result<Self::Model, RepositoryError> {
//         let res = WeaponsQueryBuilder::read_query(uuid)
//             .fetch_one(&mut *trx)
//             .await?;
//         Ok(res)
//     }

//     async fn list_trx<'b>(&self, mut trx: sqlx::Transaction<'b, MySql>) -> Result<Vec<Self::Model>, RepositoryError> {
//         let res = WeaponsQueryBuilder::list_query()
//             .fetch_all(&mut *trx)
//             .await?;

//         Ok(res)
//     }

//     async fn update_trx<'b>(&mut self, uuid: &Uuid, item: &Self::UpdateSchema, mut trx: sqlx::Transaction<'b, MySql>) -> Result<Self::Model, RepositoryError> {
//         self.query_builder
//             .update_query(uuid, item)
//             .execute(&mut *trx)
//             .await?;

//         let res = self.read_trx(uuid, trx).await?;
//         Ok(res)
//     }

//     async fn delete_trx<'b>(&mut self, uuid: &Uuid, mut trx: sqlx::Transaction<'b, MySql>) -> Result<(), RepositoryError> {
//         self.query_builder
//             .delete_query(uuid)
//             .execute(&mut *trx)
//             .await?;

//         Ok(())
//     }
// }
