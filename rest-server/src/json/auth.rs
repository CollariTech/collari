use oauth2::CsrfToken;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthUrlResponse {
    pub auth_url: String,
}

#[derive(Deserialize)]
pub struct AuthzResp {
    code: String,
    state: CsrfToken,
}

#[derive(Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}