use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{random::Random, unique_entity::UniqueEntity};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    uuid: Uuid,
}

impl Session {
    // server only
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}

impl UniqueEntity for Session {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

// server only
impl Random for Session {
    fn random() -> Self {
        Self::new(Uuid::new_v4())
    }
}
