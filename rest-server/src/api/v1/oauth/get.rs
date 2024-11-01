use crate::api::v1::oauth::{CSRF_STATE_KEY, OAUTH_METHOD_KEY, OAUTH_TOKEN_KEY, PKCE_VERIFIER_KEY};
use crate::json::auth::{AuthzResp, UserInfo};
use crate::json::{error, ok, CollariResponse};
use crate::oauth::OAuthProvider;
use crate::AppState;
use axum::extract::{Path, Query};
use axum::{Extension, Json};
use gatekeeper::middleware::common::grpc::auth::credentials::Creds;
use gatekeeper::middleware::common::grpc::auth::OauthCreds;
use oauth2::TokenResponse;
use reqwest::StatusCode;
use tower_sessions::Session;

pub async fn callback(
    session: Session,
    Path(oauth_method): Path<String>,
    Query(AuthzResp { code, state: new_state }): Query<AuthzResp>,
    Extension(providers): Extension<OAuthProvider>,
    Extension(mut state): Extension<AppState>,
) -> CollariResponse<Json<crate::json::auth::TokenResponse>> {
    if let Ok(Some(old_state)) = session.remove::<String>(CSRF_STATE_KEY).await {
        if &old_state != new_state.secret() {
            return error(StatusCode::BAD_REQUEST, "state don't match");
        };
    } else {
        return error(StatusCode::BAD_REQUEST, "state not found");
    };

    let Ok(Some(pkce)) = session.remove::<String>(PKCE_VERIFIER_KEY).await else {
        return error(StatusCode::BAD_REQUEST, "pkce not found");
    };

    let provider = providers.get_provider(&oauth_method);
    let token = provider
        .get_token(code, oauth2::PkceCodeVerifier::new(pkce))
        .await
        .access_token()
        .secret()
        .clone();

    let user_info: UserInfo = provider.get_user(&token).await;
    let creds = OauthCreds {
        token: token.clone(),
        method: oauth_method.clone(),
        email: user_info.email,
        name: user_info.name,
    };

    // check if the user has an account
    let user_token = state.client.auth_service.login(Creds::Oauth(creds)).await;

    if let Ok(response) = user_token {
        ok(
            Json(crate::json::auth::TokenResponse {
                token: Some(response.token),
            })
        )
    } else {
        session.insert(OAUTH_TOKEN_KEY, token).await.unwrap();
        session.insert(OAUTH_METHOD_KEY, oauth_method).await.unwrap();

        ok(
            Json(crate::json::auth::TokenResponse {
                token: None,
            })
        )
    }
}