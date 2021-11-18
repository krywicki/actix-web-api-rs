use async_trait::async_trait;
use jsonwebtoken::{
    errors::{Error, ErrorKind},
    Algorithm, DecodingKey, TokenData,
};
use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use crate::utils::oauth::models::{Jwk, Jwks};

use super::{DecodeInfo, Decoder};

pub struct JwksDecoder {
    jwks_provider: JwksProvider,
    secret: Option<String>,
}

impl JwksDecoder {
    async fn decode_rsa<T>(&self, info: &DecodeInfo<'_>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        let kid = info
            .header
            .kid
            .as_ref()
            .ok_or(Error::from(ErrorKind::InvalidKeyFormat))?;

        let jwks = self
            .jwks_provider
            .jwks()
            .await
            .map_err(|_| Error::from(ErrorKind::InvalidKeyFormat))?;

        let jwk = jwks
            .jwk(&kid)
            .ok_or(Error::from(ErrorKind::InvalidKeyFormat))?;

        let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e);

        jsonwebtoken::decode(&info.token, &key, &info.validation)
    }

    fn decode_hsa<T>(&self, info: &DecodeInfo<'_>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        let secret = self
            .secret
            .as_ref()
            .ok_or(Error::from(ErrorKind::InvalidKeyFormat))?;

        let key = DecodingKey::from_secret(secret.as_bytes());

        jsonwebtoken::decode(&info.token, &key, &info.validation)
    }
}

#[async_trait(?Send)]
impl Decoder for JwksDecoder {
    async fn decode<T>(&self, info: &DecodeInfo<'_>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        match info.header.alg {
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => self.decode_rsa(&info).await,
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => self.decode_hsa(&info),
            _ => Err(Error::from(ErrorKind::InvalidAlgorithm)),
        }
    }
}

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
