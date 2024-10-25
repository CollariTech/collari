mod get;
mod post;

use axum::Router;
use axum::routing::{get, post};

pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";
pub const PKCE_VERIFIER_KEY: &str = "oauth.pkce-code";

pub fn router() -> Router {
    Router::new()
        .route("/:oauth_method", post(post::oauth))
        .route("/:oauth_method/callback", get(get::callback))
}