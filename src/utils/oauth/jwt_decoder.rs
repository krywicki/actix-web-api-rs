use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    rc::Rc,
};

use super::{
    models::{Jwk, Jwks},
    OpenIdConfig,
};

use cached::{Cached, TimedCache};
use jsonwebtoken::{
    errors::{Error, ErrorKind},
    TokenData,
};
use serde::Deserialize;

pub struct JwtDecoder {
    skip_validate: bool,
    jwks_uri: String,
    jwks_cache: Rc<TimedCache<String, Jwks>>,
}

impl Default for JwtDecoder {
    fn default() -> Self {
        Self {
            skip_validate: false,
            jwks_cache: Rc::new(TimedCache::with_lifespan(3600)),
            jwks_uri: "".into(),
        }
    }
}

impl JwtDecoder {
    pub async fn from_well_known(uri: impl Into<String>) -> Result<Self, String> {
        let config = OpenIdConfig::from_well_known(uri).await?;

        Ok(Self {
            jwks_uri: config.jwks_uri,
            ..Default::default()
        })
    }

    pub async fn decode<'de, T>(token: impl Into<String>) -> Result<TokenData<T>, Error>
    where
        T: Deserialize<'de>,
    {
        Err(ErrorKind::InvalidToken.into())
    }

    async fn decode_symmetric<'de, T>(
        &self,
        token: impl Into<String>,
    ) -> Result<TokenData<T>, Error> {
        Err(ErrorKind::InvalidToken.into())
    }

    async fn decode_asymmetric<'de, T>(
        &self,
        token: impl Into<String>,
    ) -> Result<TokenData<T>, Error> {
        Err(ErrorKind::InvalidToken.into())
    }

    async fn fetch_jwks(&mut self) -> Result<&Jwks, String> {
        if let Some(jwks) = self.jwks_cache.get_mut().cache_get(&self.jwks_uri) {
            return Ok(jwks);
        } else {
            let jwks = Jwks::from_uri(self.jwks_uri.clone()).await?;
            self.jwks_cache
                .borrow_mut()
                .cache_set(self.jwks_uri.clone(), jwks);
            return Err("".into());
        }

        // let cache = &mut self.jwks_cache;
        // let jwks = Jwks::from_uri(uri.clone()).await?;
        // cache.cache_set(uri.clone(), jwks);
        // Err("".into())

        // match cache.cache_get(&self.jwks_uri) {
        //     Some(jwks) => Ok(jwks),
        //     None => {
        //         let jwks = Jwks::from_uri(self.jwks_uri).await?;
        //         cache.cache_set(self.jwks_uri.clone(), jwks);

        //         Ok(self.jwks_cache.cache_get(&self.jwks_uri).unwrap())
        //     }
        // }

        // if let Some(jwks) = self.jwks_cache.cache_get(&self.jwks_uri) {
        //     Ok(jwks)
        // } else {
        //     self.update_jwks_cache().await
        // }
    }

    async fn fetch_jwk(&mut self, kid: &str) -> Result<&Jwk, String> {
        Err("".into())
    }
}
