use sqlx::{MySql, Transaction};
use uuid::Uuid;

use crate::repository::RepositoryError;

pub trait TrxRepository {
    type Model;
    type CreateSchema;
    type UpdateSchema;

    async fn create_trx<'a>(&self, item: &Self::CreateSchema, trx: Transaction<'a, MySql>) -> Result<Self::Model, RepositoryError>;
    async fn read_trx<'a>(&self, uuid: &Uuid, trx: Transaction<'a, MySql>) -> Result<Self::Model, RepositoryError>;
    async fn list_trx<'a>(&self, trx: Transaction<'a, MySql>) -> Result<Vec<Self::Model>, RepositoryError>;
    async fn update_trx<'a>(&mut self, uuid: &Uuid, item: &Self::UpdateSchema, trx: Transaction<'a, MySql>) -> Result<Self::Model, RepositoryError>;
    async fn delete_trx<'a>(&mut self, uuid: &Uuid, trx: Transaction<'a, MySql>) -> Result<(), RepositoryError>;
}