pub mod auth;

use axum::http::StatusCode;
use axum::Json;
use axum_extra::either::Either;
use serde::Serialize;

pub type AutocondoResponse<T> = (StatusCode, Either<Json<T>, Json<AutocondoError>>);

#[derive(Debug, Serialize)]
pub struct AutocondoError {
    pub status: u16,
    pub message: String
}

pub fn ok<T: Serialize>(data: T) -> AutocondoResponse<T> {
    (StatusCode::OK, Either::E1(Json(data)))
}

pub fn no_content() -> AutocondoResponse<()> {
    (StatusCode::NO_CONTENT, Either::E1(Json(())))
}

pub fn error<T: Serialize>(status: StatusCode, message: &str) -> AutocondoResponse<T> {
    (status, Either::E2(Json(AutocondoError {
        status: status.as_u16(),
        message: String::from(message)
    })))
}