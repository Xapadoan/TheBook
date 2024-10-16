use std::path::PathBuf;

use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{tournament::contestant::TournamentContestant, unique_entity::UniqueEntity, warrior::Warrior};

use crate::repository::{FileRepository, Repository};

pub async fn remove_warrior_from_replay(
    Extension(mut warrior): Extension<Warrior>
) -> Result<Json<Value>, StatusCode> {
    warrior.set_current_tournament(None);
    let repo = match FileRepository::build(PathBuf::from("saves/warriors")) {
        Ok(repo) => repo,
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR) },
    };
    if repo.update(warrior.uuid(), &warrior).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    Ok(Json(json!(())))
}
