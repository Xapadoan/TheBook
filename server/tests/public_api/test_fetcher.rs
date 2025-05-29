use std::env;

use serde::Serialize;
use ureq::Response;
use uuid::Uuid;

fn api_url() -> String {
    let host = env::var("API_HOST").expect("Missing env API_HOST");
    let port = env::var("API_PORT").expect("Missing env API_PORT");

    format!("http://{host}:{port}")
}

pub struct TestFetcher<'a> {
    session_uuid: &'a Uuid,
    base_url: String,
}
impl<'a> TestFetcher<'a> {
    pub async fn get(&self, path: &str) -> Result<Response, ureq::Error> {
        let res = ureq::get(&format!("{}{}", self.base_url, path))
            .set("X-Session-Id", &self.session_uuid.to_string())
            .call()?;

        Ok(res)
    }
    pub async fn patch<B: Serialize>(&self, path:&str, body: B) -> Result<Response, ureq::Error> {
        let res = ureq::patch(&format!("{}{}", self.base_url, path))
            .set("X-Session-Id", &self.session_uuid.to_string())
            .set("Content-Type", "application/json")
            .send_json(body)?;
        Ok(res)
    }
    pub fn new(session_uuid: &'a Uuid) -> Self {
        Self {
            session_uuid,
            base_url: api_url(),
         }
    }
}

pub const VALID_SESSION_UUID: Uuid = Uuid::from_u128(0);

pub const INVALID_SESSION_UUID: Uuid = Uuid::from_u128(1);

pub const VALID_WARRIOR_UUID: Uuid = Uuid::from_u128(0);

pub const OPEN_TOURNAMENT_UUID: Uuid = Uuid::from_u128(0);
