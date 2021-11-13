use std::time::Duration;

use jsonwebtoken::{
    errors::{Error, ErrorKind},
    Algorithm, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;

use super::jwks::JwksProvider;

trait AlgDecoder {
    fn decode<T>(&self, token: &str, header: &Header) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned;
}

enum RsaPubKeyProvider {
    Jwks(JwksProvider),
}

struct RsaDecoder<'a> {
    pub_key_provider: RsaPubKeyProvider,
    validation: &'a Validation,
}

impl<'a> RsaDecoder<'a> {
    pub fn new(key_provider: RsaPubKeyProvider, validation: &'a Validation) -> Self {
        Self {
            pub_key_provider: key_provider,
            validation: validation,
        }
    }

    async fn decode_with_jwt<T>(
        &self,
        token: &str,
        header: &Header,
        provider: &JwksProvider,
    ) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        let f = provider
            .jwks()
            .await
            .map_err(|_| Error::from(ErrorKind::ExpiredSignature))?;

        Err(ErrorKind::ExpiredSignature.into())
    }
}

impl<'a> AlgDecoder for RsaDecoder<'a> {
    fn decode<T>(&self, token: &str, header: &Header) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        Err(ErrorKind::InvalidToken.into())
    }
}
