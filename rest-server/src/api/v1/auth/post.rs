use crate::api::v1::oauth::{OAUTH_METHOD_KEY, OAUTH_TOKEN_KEY};
use crate::json::auth::{LoginCreds, SignUpCreds, TokenResponse};
use crate::json::{error, ok, CollariResponse};
use crate::AppState;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_garde::WithValidation;
use gatekeeper::middleware::common::grpc::auth::credentials::Creds;
use gatekeeper::middleware::common::grpc::auth::{OauthCreds, PasswordCreds};
use tower_sessions::Session;

pub async fn login(
    Extension(mut state): Extension<AppState>,
    WithValidation(payload): WithValidation<Json<LoginCreds>>,
) -> CollariResponse<Json<TokenResponse>> {
    let payload = payload.into_inner();
    let creds = PasswordCreds {
        email: payload.email,
        name: None,
        password: payload.password,
    };
    
    match state.client.auth_service.login(Creds::Password(creds)).await {
        Ok(auth) => ok(Json(TokenResponse { token: Some(auth.token) })),
        Err(_) => error(StatusCode::UNAUTHORIZED, "invalid credentials"),
    }
}

pub async fn signup(
    session: Session,
    Extension(mut state): Extension<AppState>,
    WithValidation(payload): WithValidation<Json<SignUpCreds>>,
) -> CollariResponse<Json<TokenResponse>> {
    let payload = payload.into_inner();

    let oauth_token = session.remove::<String>(OAUTH_TOKEN_KEY).await;
    let oauth_method = session.remove::<String>(OAUTH_METHOD_KEY).await;

    let creds = if let (Ok(Some(oauth_token)), Ok(Some(oauth_method))) = (oauth_token, oauth_method) {
        let creds = OauthCreds {
            token: oauth_token,
            method: oauth_method,
            email: payload.email,
            name: payload.name,
        };

        Creds::Oauth(creds)
    } else {
        if let Some(password) = payload.password {
            let creds = PasswordCreds {
                email: payload.email,
                name: Some(payload.name),
                password,
            };

            Creds::Password(creds)
        } else {
            return error(StatusCode::BAD_REQUEST, "password required");
        }
    };

    let user_token = match state.client.auth_service.signup(creds).await {
        Ok(response) => response.token,
        Err(_) => return error(StatusCode::BAD_REQUEST, "user already exists"),
    };

    ok(
        Json(TokenResponse {
            token: Some(user_token),
        })
    )
}