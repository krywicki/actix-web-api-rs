use reqwest;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

pub struct JwksProviderCache {
    ttl: Duration,
    instant: Instant,
    value: Option<Rc<Jwks>>,
}

impl JwksProviderCache {
    pub fn expired(&self) -> bool {
        self.value.is_none() || self.ttl.is_zero() || self.instant.elapsed() > self.ttl
    }

    pub fn update(&mut self, jwks: Rc<Jwks>) -> Rc<Jwks> {
        self.value = Some(jwks.clone());
        jwks
    }

    pub fn value(&self) -> Option<Rc<Jwks>> {
        self.value.clone()
    }
}

pub struct JwksProvider {
    uri: String,
    cache: RefCell<JwksProviderCache>,
}

impl JwksProvider {
    pub fn new(uri: impl AsRef<str>, cache_ttl: Duration) -> Self {
        Self {
            uri: String::from(uri.as_ref()),
            cache: RefCell::new(JwksProviderCache {
                ttl: cache_ttl,
                instant: Instant::now(),
                value: None,
            }),
        }
    }

    pub fn default(uri: impl AsRef<str>) -> Self {
        Self::new(uri, Duration::from_secs(3600))
    }

    pub async fn jwks(&self) -> Result<Rc<Jwks>, String> {
        let cache = self.cache.borrow();

        if cache.expired() {
            let mut cache = self.cache.borrow_mut();

            let jwks = Rc::new(Jwks::from_uri(&self.uri).await?);
            return Ok(cache.update(jwks));
        } else {
            return Ok(cache.value().ok_or("no value")?);
        }
    }
}

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
