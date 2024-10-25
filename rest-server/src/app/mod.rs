pub mod oauth;

use axum::http::{header, Method};
use axum::{Extension, Router};
use crate::app::oauth::OAuthProvider;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub fn router() -> Router {
        Router::new()
            .nest("/api", crate::api::router())
            .layer(configure_cors())
            .layer(Extension(OAuthProvider::new()))
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            std::env::var("REST_SERVER_HOST").expect("REST_SERVER_HOST not set"),
            std::env::var("REST_SERVER_PORT").expect("REST_SERVER_PORT not set")
        ))
            .await
            .unwrap();
        axum::serve(listener, Self::router())
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