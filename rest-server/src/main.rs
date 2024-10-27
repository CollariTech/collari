use std::sync::Arc;
use axum::http::{header, Method};
use axum::{Extension, Router};
use gatekeeper::client::GatekeeperClient;
use tokio::sync::Mutex;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions::cookie::SameSite;
use crate::oauth::{OAuthProvider, Provider};

mod oauth;
mod api;
mod json;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Mutex<GatekeeperClient>>,
}

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = Arc::new(Mutex::new(
        GatekeeperClient::new("http://[::1]:10000".to_string())
    ));
    let state = AppState {
        client
    };

    let router = Router::new()
        .nest("/api", api::router())
        .layer(configure_cors())
        .layer(Extension(configure_oauth_provider()))
        .layer(configure_session())
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        std::env::var("REST_SERVER_HOST").expect("REST_SERVER_HOST not set"),
        std::env::var("REST_SERVER_PORT").expect("REST_SERVER_PORT not set")
    ))
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .expect("Failed to start axum server");
}

fn configure_oauth_provider() -> OAuthProvider {
    OAuthProvider::new(
        Provider::new(
            oauth::new_client(
                std::env::var("GOOGLE_CLIENT_ID").expect(""),
                std::env::var("GOOGLE_CLIENT_SECRET").expect(""),
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://www.googleapis.com/oauth2/v3/token",
                "",
            ),
            "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
            vec!["email", "profile"],
        ),
        Provider::new(
            oauth::new_client(
                std::env::var("FACEBOOK_CLIENT_ID").expect(""),
                std::env::var("FACEBOOK_CLIENT_SECRET").expect(""),
                "https://www.facebook.com/v20.0/dialog/oauth",
                "https://graph.facebook.com/oauth/access_token",
                "",
            ),
            "https://graph.facebook.com/me?fields=name,email".to_string(),
            vec!["email"],
        ),
    )
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