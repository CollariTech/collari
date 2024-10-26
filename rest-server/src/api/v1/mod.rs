mod oauth;
mod auth;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/oauth", oauth::router())
}