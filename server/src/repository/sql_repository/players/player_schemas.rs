use uuid::Uuid;

pub struct CreatePlayerSchema {
    pub uuid: Uuid,
    pub username: String,
}
impl CreatePlayerSchema {
    pub fn new(username: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username: username.to_string(),
        }
    }
}

pub struct UpdatePlayerSchema {}