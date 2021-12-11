use crate::response_ext::ResponseExt;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Deserialize;
use surf::{
    http::{Method, Url},
    Request,
};

mod claims;
use claims::Claims;
pub mod errors;
use errors::Result;
mod response_ext;

const ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

fn generate_jwt<T>(app_id: usize, rsa_key_path: T) -> Result<String>
where
    T: AsRef<std::path::Path>,
{
    let claim = Claims::new(
        Utc::now() - Duration::seconds(10),
        Utc::now() + Duration::minutes(10),
        app_id,
    );
    let header = Header::new(Algorithm::RS256);
    let key = std::fs::read_to_string(rsa_key_path)?;
    let token = encode(&header, &claim, &EncodingKey::from_rsa_pem(key.as_bytes())?)?;

    Ok(token)
}

#[derive(Debug, Deserialize)]
struct Account {
    login: String,
}

#[derive(Debug, Deserialize)]
struct InstalledApp {
    account: Account,
    access_tokens_url: String,
}

async fn fetch_installed_apps(token: &str) -> Result<Vec<InstalledApp>> {
    let url = Url::parse("https://api.github.com/app/installations")?;
    let request = Request::builder(Method::Get, url)
        .header("Accept", ACCEPT_HEADER)
        .header("Authorization", format!("Bearer {}", token))
        .build();
    let mut response = surf::client().send(request).await.convert()?;
    let apps: Vec<InstalledApp> = response.body_json().await?;

    Ok(apps)
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

async fn fetch_token(token: &str, url: &str) -> Result<Token> {
    let url = Url::parse(url)?;
    let request = Request::builder(Method::Post, url)
        .header("Accept", ACCEPT_HEADER)
        .header("Authorization", format!("Bearer {}", token))
        .build();
    let mut token = surf::client().send(request).await.convert()?;
    let token: Token = token.body_json().await?;

    Ok(token)
}

pub async fn publish_token<T>(
    app_id: usize,
    rsa_key_path: T,
    repository_owner: &str,
) -> Result<Token>
where
    T: AsRef<std::path::Path>,
{
    let jwt = generate_jwt(app_id, rsa_key_path)?;
    let apps = fetch_installed_apps(&jwt).await?;
    let inst_url = apps
        .iter()
        .find(|app| app.account.login == repository_owner)
        .map(|app| app.access_tokens_url.to_owned())
        .ok_or_else(|| errors::new_error(errors::ErrorKind::InstallationIdNotFound))?;

    Ok(fetch_token(&jwt, &inst_url).await?)
}
