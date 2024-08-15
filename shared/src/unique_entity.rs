use uuid::Uuid;

pub trait UniqueEntity {
    fn uuid(&self) -> &Uuid;
}
