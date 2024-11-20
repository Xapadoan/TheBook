use std::sync::Arc;

use serde_json::json;
use axum::{http::StatusCode, routing::get, Json, Router};
use sqlx::MySqlPool;

use crate::repository::sql_repository::gen_pool;

#[derive(Debug, Clone)]
pub struct AppState {
    db_pool: MySqlPool,
}

impl AppState {
    pub fn db_pool(&self) -> &MySqlPool {
        &self.db_pool
    }
}

pub async fn run_server() {
    let port = std::env::var("PORT").expect("Missing Env: PORT");
    let db_pool = gen_pool().await;
    let app_state = Arc::new(AppState { db_pool: db_pool.clone() });
    let app = Router::new()
        .route("/ping", get(|| async move {
            eprintln!("[DEBUG] /ping");
            Json(json!(()))
        }))
        .route("/pong", get(|| async move {
            eprintln!("[DEBUG] /pong");
            Json(json!(()))
        }))
        .nest("/auth", super::auth::auth_routes())
        .nest("/shop", super::shop::shop_routes())
        .nest("/player", super::player::player_routes(app_state.clone()))
        .nest("/tournaments", super::tournaments::tournaments_routes())
        .nest("/replays", super::replays::replay_routes())
        .with_state(app_state.clone());
    let listener = tokio::net::TcpListener::bind(
        &format!("0.0.0.0:{port}")
    )
        .await.unwrap();
    eprintln!("[DEBUG] TCP Listener up on 0.0.0.0:{port}");

    axum::serve(listener, app.into_make_service() ).await.unwrap();
}
