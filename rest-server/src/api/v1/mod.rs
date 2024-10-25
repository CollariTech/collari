mod oauth;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/oauth", oauth::router())
}