use crate::api::v1::oauth::{CSRF_STATE_KEY, PKCE_VERIFIER_KEY};
use crate::app::oauth::OAuthProvider;
use crate::json::auth::AuthUrlResponse;
use crate::json::{ok, AutocondoResponse};
use axum::extract::Path;
use axum::Extension;
use tower_sessions::Session;

pub async fn oauth(
    Extension(providers): Extension<OAuthProvider>,
    Path(oauth_method): Path<String>,
    session: Session,
) -> AutocondoResponse<AuthUrlResponse> {
    let ((auth_url, csrf), pkce_verifier) = providers.get_provider(&oauth_method)
        .auth_url();

    session
        .insert(CSRF_STATE_KEY, csrf.secret())
        .await
        .unwrap();

    session
        .insert(PKCE_VERIFIER_KEY, pkce_verifier.secret())
        .await
        .unwrap();

    ok(AuthUrlResponse {
        auth_url: auth_url.to_string(),
    })
}