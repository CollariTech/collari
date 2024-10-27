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

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        std::env::var("REST_SERVER_HOST").expect("REST_SERVER_HOST not set"),
        std::env::var("REST_SERVER_PORT").expect("REST_SERVER_PORT not set")
    ))
        .await
        .unwrap();
    axum::serve(listener, api::router(state, configure_oauth_provider()))
        .await
        .expect("Failed to start axum server");
}

fn configure_oauth_provider() -> OAuthProvider {
    OAuthProvider::new(
        Provider::new(
            oauth::new_client(
                std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set"),
                std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not set"),
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://www.googleapis.com/oauth2/v3/token",
                "",
            ),
            "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
            vec!["email", "profile"],
        ),
        Provider::new(
            oauth::new_client(
                std::env::var("FACEBOOK_CLIENT_ID").expect("FACEBOOK_CLIENT_ID not set"),
                std::env::var("FACEBOOK_CLIENT_SECRET").expect("FACEBOOK_CLIENT_SECRET not set"),
                "https://www.facebook.com/v20.0/dialog/oauth",
                "https://graph.facebook.com/oauth/access_token",
                "",
            ),
            "https://graph.facebook.com/me?fields=name,email".to_string(),
            vec!["email"],
        ),
    )
}

