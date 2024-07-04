use std::error::Error;

use uuid::Uuid;

pub trait UniqueEntity {
    fn uuid<'a>(&'a self) -> &'a Uuid;
}

pub trait Repository<T> {
    fn create(&self, item: &T) -> Result<(), Box<dyn Error>>;
    fn get_by_uuid(&self, uuid: &Uuid) -> Result<T, Box<dyn Error>>;
}
