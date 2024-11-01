mod post;
mod get;

use axum::Router;
use axum::routing::{get, post};
use gatekeeper::middleware::login_required;

pub fn router() -> Router {
    Router::new()
        .route("/logout", get(get::logout))
        .layer(login_required!())
        .route("/signup", post(post::signup))
        .route("/login", post(post::login))
}