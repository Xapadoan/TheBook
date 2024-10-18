use std::{fmt::Display, io};

use serde::{de::DeserializeOwned, Serialize};

use dotenv;
use shared::{auth::Session, replay::FightReplaySummary, unique_entity::UniqueEntity};

pub struct ApiFetcher<'a> {
    hostname: String,
    session: &'a Session,
}

impl<'a> ApiFetcher<'a> {
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiFetcherError> {
        let value = ureq::get(self.full_path(path).as_str())
            .set("X-Session-Id", &self.session.uuid().to_string())
            .call()?
            .into_json::<T>()?;
        Ok(value)
    }

    pub fn patch<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: B) -> Result<T, ApiFetcherError> {
        let value = ureq::patch(self.full_path(path).as_str())
            .set("X-Session-Id", &self.session.uuid().to_string())
            .set("Content-Type", "application/json")
            .send_json(body)?
            .into_json::<T>()?;
        Ok(value)
    }

    pub fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: B) -> Result<T, ApiFetcherError> {
        let value = ureq::post(self.full_path(path).as_str())
            .set("X-Session-Id", &self.session.uuid().to_string())
            .set("Content-Type", "application/json")
            .send_json(body)?
            .into_json::<T>()?;
        Ok(value)
    }

    pub fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiFetcherError> {
        let value = ureq::delete(self.full_path(path).as_str())
            .set("X-Session-Id", &self.session.uuid().to_string())
            .call()?
            .into_json::<T>()?;
        Ok(value)
    }

    pub fn new(session: &'a Session) -> Self {
        let backend_url = dotenv::var("BACKEND_URL");
        assert!(
            backend_url.is_ok(),
            "Missing required env BACKEND_URL",
        );
        ApiFetcher { hostname: backend_url.unwrap(), session }
    }

    fn full_path(&self, path: &str) -> String {
        self.hostname.clone() + path
    }
}

pub enum ApiFetcherError {
    UReq(ureq::Error),
    Io(io::Error),
}

impl From<ureq::Error> for ApiFetcherError {
    fn from(value: ureq::Error) -> Self {
        Self::UReq(value)
    }
}

impl From<io::Error> for ApiFetcherError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for ApiFetcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UReq(e) => write!(f, "Fetch error:\n {e:#?}"),
            Self::Io(e) => write!(f, "Deserialization Error:\n {e:#?}"),
        }
    }
}

pub trait ToQueryString {
    fn to_query_string(&self) -> String;
}

impl ToQueryString for FightReplaySummary {
    fn to_query_string(&self) -> String {
        let mut str = String::new();
        str += format!("replay_uuid={}", self.replay_uuid().to_string()).as_str();
        str += format!("&blue_corner_uuid={}", self.blue_corner_uuid().to_string()).as_str();
        str += format!("&red_corner_uuid={}", self.red_corner_uuid().to_string()).as_str();
        str += "&winner=";
        if let Some(winner) = self.winner() {
            str += winner.to_string().as_str();
        }

        str
    }
}
