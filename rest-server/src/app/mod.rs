pub mod oauth;

use crate::app::oauth::OAuthProvider;
use axum::http::{header, Method};
use axum::{Extension, Router};
use gatekeeper::client::GatekeeperClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_sessions::cookie::SameSite;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Mutex<GatekeeperClient>>,
}

pub struct Server {
    state: AppState,
}

impl Server {
    pub fn new() -> Self {
        let client = Arc::new(Mutex::new(
            GatekeeperClient::new("http://[::1]:10000".to_string())
        ));

        Self {
            state: AppState {
                client
            }
        }
    }

    pub fn router(state: AppState) -> Router {
        Router::new()
            .nest("/api", crate::api::router())
            .layer(configure_cors())
            .layer(Extension(OAuthProvider::new()))
            .layer(configure_session())
            .layer(Extension(state))
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            std::env::var("REST_SERVER_HOST").expect("REST_SERVER_HOST not set"),
            std::env::var("REST_SERVER_PORT").expect("REST_SERVER_PORT not set")
        ))
            .await
            .unwrap();
        axum::serve(listener, Self::router(self.state))
            .await
            .expect("Failed to start axum server");
    }
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