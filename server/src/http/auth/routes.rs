use std::sync::Arc;

use axum::{routing::post, Router};

use crate::http::app::AppState;

use super::signup::signup;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/signup", post(signup))
}
