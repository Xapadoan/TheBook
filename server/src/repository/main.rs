use std::error::Error;
use std::fmt::Display;

use uuid::Uuid;

pub trait RepositoryCreate<T, K> {
    async fn create(&self, item: &K) -> Result<T, RepositoryError>;
}
pub trait RepositoryRead<T> {
    async fn read(&self, uuid: &Uuid) -> Result<T, RepositoryError>;
}
pub trait RepositoryList<T> {
    async fn list(&self) -> Result<Vec<T>, RepositoryError>;
}
pub trait RepositoryUpdate<T, K> {
    async fn update(&self, uuid: &Uuid, item: &K) -> Result<T, RepositoryError>;
}
pub trait RepositoryDelete {
    async fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError>;
}

pub trait Repository:
    RepositoryCreate<Self::Model, Self::CreateSchema> +
    RepositoryRead<Self::Model> +
    RepositoryList<Self::Model> +
    RepositoryUpdate<Self::Model, Self::UpdateSchema> +
    RepositoryDelete
{
    type Model;
    type CreateSchema;
    type UpdateSchema;

    // async fn list(&self) -> Result<Vec<Self::Model>, RepositoryError>;
    // async fn create(&self, item: &Self::CreateSchema) -> Result<Self::Model, RepositoryError>;
    // async fn read(&self, uuid: &Uuid) -> Result<Self::Model, RepositoryError>;
    // async fn update(&self, uuid: &Uuid, item: &Self::UpdateSchema) -> Result<Self::Model, RepositoryError>;
    // async fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError>;
}

#[derive(Debug)]
pub struct RepositoryError {
    message: String,
}

impl RepositoryError {
    pub fn new(message: String) -> Self {
        Self {
            message: format!("Repository Error:\n{message}")
        }
    }
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RepositoryError {}

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::new("RowNotFound".to_string()),
            other => Self::new(format!("Unknown Error:\n{:?}", other))
        }
    }
}
