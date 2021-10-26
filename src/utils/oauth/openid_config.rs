use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenIdConfig {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub request_object_signing_alg_values_supported: Vec<String>,
    pub jwks_uri: String,
}

impl OpenIdConfig {
    pub async fn from_well_known(uri: impl Into<String>) -> Result<Self, String> {
        let resp = reqwest::get(uri.into()).await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }
}
