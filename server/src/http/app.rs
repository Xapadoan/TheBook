use axum::Router;

#[tokio::main]
pub async fn run_server() {
    let app = Router::new()
        .nest("/shop", super::shop::shop_routes())
        .nest("/player", super::player::player_routes())
        .nest("/tournaments", super::tournaments::tournaments_routes())
        .nest("/replays", super::replays::replay_routes());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
