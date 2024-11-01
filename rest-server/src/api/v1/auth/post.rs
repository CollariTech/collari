use crate::api::v1::oauth::{OAUTH_METHOD_KEY, OAUTH_TOKEN_KEY};
use crate::json::auth::{LoginCreds, SignUpCreds};
use crate::json::{error, CollariResponse};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_garde::WithValidation;
use gatekeeper::middleware::common::grpc::auth::credentials::Creds;
use gatekeeper::middleware::common::grpc::auth::{OauthCreds, PasswordCreds};
use tower_sessions::Session;
use crate::AppState;

pub async fn login(
    Extension(state): Extension<AppState>,
    WithValidation(payload): WithValidation<Json<LoginCreds>>,
) -> CollariResponse<()> {
    let payload = payload.into_inner();
    let creds = PasswordCreds {
        email: payload.email,
        name: "".to_string(),
        password: payload.password,
    };

    let mut client = state.client.lock().await;
    client.login(Creds::Password(creds)).await.unwrap();

    todo!()
}

pub async fn signup(
    session: Session,
    Extension(state): Extension<AppState>,
    WithValidation(payload): WithValidation<Json<SignUpCreds>>,
) -> CollariResponse<()> {
    let payload = payload.into_inner();

    let oauth_token = session.remove::<String>(OAUTH_TOKEN_KEY).await;
    let oauth_method = session.remove::<String>(OAUTH_METHOD_KEY).await;

    let mut client = state.client.lock().await;

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
                name: payload.name,
                password,
            };

            Creds::Password(creds)
        } else {
            return error(StatusCode::BAD_REQUEST, "password required");
        }
    };

    client.signup(creds).await.unwrap();

    todo!()
}