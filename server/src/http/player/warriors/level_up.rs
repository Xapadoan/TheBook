use std::path::PathBuf;

use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{
    experience::GainExperience,
    stats::StatKind,
    unique_entity::UniqueEntity, warrior::Warrior,
};

use crate::repository::{FileRepository, Repository};

#[axum::debug_handler]
pub async fn level_up(
    Extension(mut warrior): Extension<Warrior>,
    Json(stat): Json<StatKind>,
) -> Result<Json<Value>, StatusCode> {
    if let Err(_) = warrior.level_up(&stat) {
        return Err(StatusCode::CONFLICT)
    }
    let repo = match FileRepository::build(PathBuf::from("saves/warriors")) {
        Ok(repo) => repo,
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR) },
    };
    if repo.update(warrior.uuid(), &warrior).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    Ok(Json(json!(())))
}
