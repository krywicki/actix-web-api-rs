use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    rc::Rc,
    time::{Duration, Instant},
};

use super::{
    models::{Jwk, Jwks},
    OpenIdConfig,
};

use cached::{Cached, TimedCache};
use jsonwebtoken::{
    errors::{Error, ErrorKind},
    Algorithm, DecodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize};

pub struct JwtDecoder {
    skip_validate: bool,
    validation: Validation,
    cache: RefCell<JwksCache>,
}

impl Default for JwtDecoder {
    fn default() -> Self {
        Self {
            skip_validate: false,
            cache: RefCell::new(JwksCache::new("".into(), Duration::from_secs(3600))),
            validation: Validation::default(),
        }
    }
}

impl JwtDecoder {
    pub async fn from_well_known(uri: String) -> Result<Self, String> {
        let config = OpenIdConfig::from_well_known(uri).await?;

        Ok(Self {
            cache: RefCell::new(JwksCache::new(config.jwks_uri, Duration::from_secs(3600))),
            ..Default::default()
        })
    }

    pub async fn decode<'de, T>(&self, token: impl AsRef<str>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        if self.skip_validate {
            jsonwebtoken::dangerous_insecure_decode(token.as_ref())
        } else {
            let header = jsonwebtoken::decode_header(token.as_ref())?;

            match header.alg {
                Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                    return self.decode_rsa(token.as_ref(), &header).await
                }
                _ => return Err(ErrorKind::InvalidAlgorithm.into()),
            }
        }
    }

    async fn jwks(&self) -> Result<Rc<Jwks>, Error> {
        let jwks = self
            .cache
            .borrow_mut()
            .jwks()
            .await
            .map_err(|_| Error::from(ErrorKind::InvalidKeyFormat))?;
        Ok(jwks)
    }

    pub async fn decode_rsa<'de, T>(
        &self,
        token: &str,
        header: &Header,
    ) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        let kid = header
            .kid
            .as_ref()
            .ok_or(Error::from(ErrorKind::InvalidKeyFormat))?;

        let jwks = self.jwks().await?;

        let jwk = jwks
            .jwk(kid)
            .ok_or(Error::from(ErrorKind::InvalidKeyFormat))?;

        let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e);

        jsonwebtoken::decode::<T>(&token, &key, &self.validation)
    }
}

struct JwksCache {
    uri: String,
    ttl: Duration,
    instant: Instant,
    cache: Option<Rc<Jwks>>,
}

impl<'a> JwksCache {
    pub fn new(uri: String, ttl: Duration) -> Self {
        Self {
            uri: uri,
            ttl: ttl,
            instant: Instant::now(),
            cache: None,
        }
    }

    pub fn cache_expired(&self) -> bool {
        self.cache.is_none() || self.instant.elapsed() > self.ttl
    }

    pub async fn update_cache(&mut self) -> Result<(), String> {
        self.cache = Some(Rc::new(Jwks::from_uri(&self.uri).await?));
        Ok(())
    }

    pub async fn jwks(&mut self) -> Result<Rc<Jwks>, String> {
        if self.cache_expired() {
            self.update_cache().await?;
        }

        if let Some(ref jwks) = self.cache {
            Ok(jwks.clone())
        } else {
            Err("failed to update jwks cache".into())
        }
    }
}
