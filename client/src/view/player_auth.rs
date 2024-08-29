use server::api;
use shared::auth::Session;

use crate::auth::{read_session, store_session};
use crate::prompt::prompt;

use super::view_error::ViewError;

pub fn authenticate_player() -> Result<Session, ViewError> {
    if let Some(session) = read_session()? {
        Ok(session)
    } else {
        let session = signup_view()?;
        store_session(&session)?;
        Ok(session)
    }
}

fn signup_view() -> Result<Session, ViewError> {
    let username = prompt("Choose a username:")?;
    let display_name = prompt("Choose a display_name")?;
    let session = api::auth::signup(username, display_name)?;
    Ok(session)
}
