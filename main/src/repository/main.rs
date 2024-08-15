use std::error::Error;
use std::fmt::Display;

use uuid::Uuid;

pub trait UniqueEntity {
    fn uuid<'a>(&'a self) -> &'a Uuid;
}

pub trait Repository<T> {
    fn list(&self) -> Result<Vec<Uuid>, RepositoryError>;
    fn create(&self, item: &T) -> Result<(), RepositoryError>;
    fn get_by_uuid(&self, uuid: &Uuid) -> Result<T, RepositoryError>;
    fn update(&self, uuid: &Uuid, item: &T) -> Result<(), RepositoryError>;
    fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError>;
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
