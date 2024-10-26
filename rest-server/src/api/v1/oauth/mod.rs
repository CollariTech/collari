mod get;
mod post;

use axum::Router;
use axum::routing::{get, post};

const CSRF_STATE_KEY: &str = "oauth.csrf-state";
const PKCE_VERIFIER_KEY: &str = "oauth.pkce-code";
pub const OAUTH_TOKEN_KEY: &str = "oauth.token";
pub const OAUTH_METHOD_KEY: &str = "oauth.method";

pub fn router() -> Router {
    Router::new()
        .route("/:oauth_method", post(post::oauth))
        .route("/:oauth_method/callback", get(get::callback))
}