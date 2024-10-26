use crate::api::v1::oauth::{CSRF_STATE_KEY, PKCE_VERIFIER_KEY};
use crate::app::oauth::OAuthProvider;
use crate::json::{ok, CollariResponse};
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};
use axum::Extension;
use tower_sessions::Session;

pub async fn oauth(
    session: Session,
    Path(oauth_method): Path<String>,
    Extension(providers): Extension<OAuthProvider>,
) -> CollariResponse<impl IntoResponse> {
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

    ok(Redirect::to(auth_url.as_str()).into_response())
}