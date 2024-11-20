use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use shared::unique_entity::UniqueEntity;

use crate::{
    auth::{SessionManager, SignUp},
    http::app::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SignUpPayload {
    username: String,
}

#[axum::debug_handler]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<Value>, StatusCode> {
    let signer = SignUp::new(
        state.db_pool(),
        &payload.username,
    );
    let player = signer.gen_player().await;
    if let Err(err) = player {
        eprintln!("[ERROR][404] /auth/signup:\n{err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let player = player.unwrap();
    let session_manager = SessionManager::build();
    if let Err(err) = session_manager {
        eprintln!("[ERROR][404] /auth/signup:\n{err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let session_manager = session_manager.unwrap();
    let session = session_manager.create_session(player.uuid()).await;
    if let Err(err) = session {
        eprintln!("[ERROR][404] /auth/signup:\n{err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(json!(session.unwrap())))
}