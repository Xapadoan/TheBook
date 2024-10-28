use std::sync::Arc;

use axum::Router;
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

#[tokio::main]
pub async fn run_server() {
    let port = std::env::var("PORT").expect("Missing Env: PORT");
    let db_pool = gen_pool().await;
    let app = Router::new()
        .nest("/auth", super::auth::auth_routes())
        .nest("/shop", super::shop::shop_routes())
        .nest("/player", super::player::player_routes())
        .nest("/tournaments", super::tournaments::tournaments_routes())
        .nest("/replays", super::replays::replay_routes())
        .with_state(Arc::new(AppState { db_pool: db_pool.clone() }));
    let listener = tokio::net::TcpListener::bind(
        &format!("0.0.0.0:{port}")
    )
        .await.unwrap();

    axum::serve(listener, app.into_make_service() ).await.unwrap();
}
