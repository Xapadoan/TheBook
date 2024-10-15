use axum::{extract::{Path, Request}, http::StatusCode, middleware::Next, response::Response, Extension};
use shared::{player::Player, unique_entity::UniqueEntity, warrior::WarriorCollection};
use uuid::Uuid;

pub async fn player_warrior(
    Extension(player): Extension<Player>,
    Path(warrior_uuid): Path<Uuid>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    match player.warriors().iter().find(
        |w| *w.uuid() == warrior_uuid
    ) {
        None => Err(StatusCode::NOT_FOUND),
        Some(warrior) => {
            req.extensions_mut().insert(warrior.clone());
            Ok(next.run(req).await)
        }
    }

}