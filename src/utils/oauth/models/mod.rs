use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

impl Jwks {
    pub async fn from_uri(uri: impl Into<String>) -> Result<Self, String> {
        let resp = reqwest::get(uri.into()).await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Jwk {
    pub kty: String,
    pub alg: String,
    pub kid: String,
    pub e: String,
    pub n: String,
}
