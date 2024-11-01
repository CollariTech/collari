use axum::http::{header, Method};
use axum::{Extension, Router};
use gatekeeper::middleware::AuthenticationLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions::cookie::SameSite;
use crate::AppState;
use crate::oauth::OAuthProvider;

mod v1;

pub fn router(
    state: AppState,
    oauth_provider: OAuthProvider
) -> Router {
    Router::new()
        .layer(configure_cors())
        .layer(Extension(oauth_provider))
        .layer(configure_session())
        .nest("/api", Router::new()
            .nest("/v1", v1::router())
        )
        .layer(AuthenticationLayer::new(state.client.auth_service.clone()))
        .layer(Extension(state))
}

fn configure_cors() -> tower_http::cors::CorsLayer {
    tower_http::cors::CorsLayer::new()
        .allow_credentials(true)
        .allow_origin([])
        .allow_headers([
            header::CONTENT_TYPE
        ])
        .allow_methods([
            Method::GET, Method::PUT,
            Method::DELETE, Method::PATCH
        ])
}

fn configure_session() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax) // ensure we send the cookie from the OAuth redirect.
        .with_expiry(tower_sessions::Expiry::OnSessionEnd)
}