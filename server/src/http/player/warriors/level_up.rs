use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use shared::{
    experience::GainExperience,
    stats::StatKind,
    unique_entity::UniqueEntity, warrior::Warrior,
};

use crate::{
    http::app::AppState,
    repository::{
        sql_repository::warriors::{UpdateWarriorSchema, WarriorsRepository},
        RepositoryUpdate,
    },
};

#[axum::debug_handler]
pub async fn level_up(
    Extension(mut warrior): Extension<Warrior>,
    State(state): State<Arc<AppState>>,
    Json(stat): Json<StatKind>,
) -> Result<Json<Value>, StatusCode> {
    if let Err(_) = warrior.level_up(&stat) {
        return Err(StatusCode::CONFLICT)
    }
    let uuid = warrior.uuid().clone();
    let schema = UpdateWarriorSchema::from(warrior);
    let repo = WarriorsRepository::new(state.db_pool());
    if repo.update(&uuid, &schema).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    Ok(Json(json!(())))
}
