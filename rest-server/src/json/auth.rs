use garde::Validate;
use oauth2::CsrfToken;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthzResp {
    pub code: String,
    pub state: CsrfToken,
}

#[derive(Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct SignUpCreds {
    #[garde(skip)]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8, max = 64))]
    pub password: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginCreds {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8, max = 64))]
    pub password: String,
}