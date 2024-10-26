mod post;
mod get;

use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router {
    Router::new()
        .route("/logout", get(get::logout))
        .route("/signup", post(post::signup))
        .route("/login", post(post::login))
}