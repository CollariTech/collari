use crate::api::v1::oauth::{CSRF_STATE_KEY, PKCE_VERIFIER_KEY};
use crate::app::oauth::OAuthProvider;
use crate::json::auth::{AuthzResp, UserInfo};
use crate::json::{error, AutocondoResponse};
use axum::extract::{Path, Query};
use axum::Extension;
use oauth2::TokenResponse;
use reqwest::StatusCode;
use tower_sessions::Session;

pub async fn callback(
    session: Session,
    Path(oauth_method): Path<String>,
    Query(AuthzResp { code, state: new_state }): Query<AuthzResp>,
    Extension(providers): Extension<OAuthProvider>,
) -> AutocondoResponse<()> {
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

    todo!()
}