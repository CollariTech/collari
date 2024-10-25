use oauth2::basic::BasicTokenResponse;
use oauth2::Scope;

#[derive(Clone)]
pub struct OAuthProvider {
    pub google: Provider,
    pub facebook: Provider,
}

#[derive(Clone)]
pub struct Provider {
    pub client: OauthClient,
    pub info: String,
    scopes: Vec<Scope>,
}

impl OAuthProvider {
    pub fn new() -> Self {
        Self {
            google: Provider::new(
                oauth_client(
                    std::env::var("GOOGLE_CLIENT_ID").expect(""),
                    std::env::var("GOOGLE_CLIENT_SECRET").expect(""),
                    "https://accounts.google.com/o/oauth2/v2/auth",
                    "https://www.googleapis.com/oauth2/v3/token",
                    "",
                ),
                "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
                vec!["email", "profile"],
            ),
            facebook: Provider::new(
                oauth_client(
                    std::env::var("FACEBOOK_CLIENT_ID").expect(""),
                    std::env::var("FACEBOOK_CLIENT_SECRET").expect(""),
                    "https://www.facebook.com/v20.0/dialog/oauth",
                    "https://graph.facebook.com/oauth/access_token",
                    "",
                ),
                "https://graph.facebook.com/me?fields=name,email".to_string(),
                vec!["email"],
            ),
        }
    }

    pub fn get_provider(&self, oauth_type: &str) -> &Provider {
        match oauth_type {
            "google" => &self.google,
            "facebook" => &self.facebook,
            _ => &self.google
        }
    }
}

impl Provider {
    pub fn new(client: OauthClient, info: String, scopes: Vec<&str>) -> Self {
        Self {
            client,
            info,
            scopes: scopes.iter().map(|s| Scope::new(s.to_string())).collect(),
        }
    }

    pub fn auth_url(&self) -> ((oauth2::url::Url, oauth2::CsrfToken), oauth2::PkceCodeVerifier) {
        let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

        (
            self.client
                .authorize_url(oauth2::CsrfToken::new_random)
                .set_pkce_challenge(pkce_code_challenge)
                .add_scopes(self.scopes.clone())
                .url(),
            pkce_code_verifier
        )
    }

    pub async fn get_token(
        &self, 
        code: String,
        pkce_code_verifier: oauth2::PkceCodeVerifier
    ) -> BasicTokenResponse {
        self.client
            .exchange_code(oauth2::AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(
                &reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::limited(1))
                    .build().unwrap()
            )
            .await
            .unwrap()
    }

    pub async fn get_user<T: serde::de::DeserializeOwned>(&self, token: &String) -> T {
        reqwest::Client::new()
            .get(&self.info)
            .bearer_auth(token)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

fn oauth_client(
    client_id: String,
    client_secret: String,
    auth_url: &str,
    token_url: &str,
    redirect_url: &str,
) -> OauthClient {
    oauth2::basic::BasicClient::new(oauth2::ClientId::new(client_id))
        .set_client_secret(oauth2::ClientSecret::new(client_secret))
        .set_auth_uri(oauth2::AuthUrl::new(auth_url.to_string()).unwrap())
        .set_token_uri(oauth2::TokenUrl::new(token_url.to_string()).unwrap())
        .set_redirect_uri(oauth2::RedirectUrl::new(redirect_url.to_string()).unwrap())
}

type OauthClient = oauth2::Client<oauth2::basic::BasicErrorResponse, oauth2::basic::BasicTokenResponse, oauth2::basic::BasicTokenIntrospectionResponse, oauth2::StandardRevocableToken, oauth2::basic::BasicRevocationErrorResponse, oauth2::EndpointSet, oauth2::EndpointNotSet, oauth2::EndpointNotSet, oauth2::EndpointNotSet, oauth2::EndpointSet>;