use serde::Serialize;
use shared::auth::Session;

use crate::auth::{read_session, store_session};
use crate::prompt::prompt;

use super::view_error::ViewError;

pub fn authenticate_player() -> Result<Session, ViewError> {
    if let Some(session) = read_session()? {
        Ok(session)
    } else {
        let session = signup_view()?;
        eprintln!("[DEBUG] SignUp OK");
        store_session(&session)?;
        Ok(session)
    }
}

#[derive(Debug, Serialize)]
pub struct SignUpPayload {
    username: String,
}

fn signup_view() -> Result<Session, ViewError> {
    let username = prompt("Choose a username:")?;
    let backend_url = dotenv::var("BACKEND_URL")?;
    let session = ureq::post(
        format!("{backend_url}/auth/signup").as_str()
    ).set("Content-Type", "application/json")
            .send_json(SignUpPayload {
                username,
            })?
            .into_json::<Session>()?;
    Ok(session)
}
