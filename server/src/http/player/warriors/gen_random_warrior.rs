use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::{json, Value};
use shared::player::Player;
use shared::unique_entity::UniqueEntity;

use crate::http::app::AppState;
use crate::repository::sql_repository::warriors::WarriorsRepository;

pub async fn gen_random_warrior(
    Extension(player): Extension<Player>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let repo = WarriorsRepository::new(state.db_pool());
    let created_warrior = repo.create_random(player.uuid()).await;
    if let Err(err) = created_warrior {
        eprintln!("[ERROR] Failed to gen random warrior:\n{err:?}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(json!(created_warrior.unwrap())))
}
