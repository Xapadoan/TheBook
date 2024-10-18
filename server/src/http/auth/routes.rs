use axum::{routing::post, Router};

use super::signup::signup;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
}
