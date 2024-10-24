use axum::Router;

#[tokio::main]
pub async fn run_server() {
    let port = std::env::var("PORT").expect("Missing Env: PORT");
    let app = Router::new()
        .nest("/auth", super::auth::auth_routes())
        .nest("/shop", super::shop::shop_routes())
        .nest("/player", super::player::player_routes())
        .nest("/tournaments", super::tournaments::tournaments_routes())
        .nest("/replays", super::replays::replay_routes());
    let listener = tokio::net::TcpListener::bind(
        &format!("0.0.0.0:{port}")
    )
        .await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
