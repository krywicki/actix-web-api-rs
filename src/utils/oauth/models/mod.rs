use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

impl Jwks {
    pub async fn from_uri(uri: impl AsRef<str>) -> Result<Self, String> {
        let resp = reqwest::get(uri.as_ref())
            .await
            .map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub fn jwk(&self, kid: impl AsRef<str>) -> Option<&Jwk> {
        self.keys.iter().find(|k| k.kid == kid.as_ref())
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
