pub mod auth;

use axum::http::StatusCode;
use axum::Json;
use axum_extra::either::Either;
use serde::Serialize;

pub type CollariResponse<T> = (StatusCode, Either<T, Json<CollariError>>);

#[derive(Debug, Serialize)]
pub struct CollariError {
    pub status: u16,
    pub message: String
}

pub fn ok<T>(data: T) -> CollariResponse<T> {
    (StatusCode::OK, Either::E1(data))
}

pub fn no_content() -> CollariResponse<()> {
    (StatusCode::NO_CONTENT, Either::E1(()))
}

pub fn error<T>(status: StatusCode, message: &str) -> CollariResponse<T> {
    (status, Either::E2(Json(CollariError {
        status: status.as_u16(),
        message: String::from(message)
    })))
}