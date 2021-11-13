use actix_web::web::json;
use async_trait::async_trait;
use jsonwebtoken::{
    errors::{Error, ErrorKind},
    Algorithm, DecodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;

use super::jwks::JwksProvider;

pub struct JwtDecoder<D>
where
    D: Decoder,
{
    validation: Validation,
    decoder: D,
}

impl<D> JwtDecoder<D>
where
    D: Decoder,
{
    async fn decode<T>(&self, token: &str) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
    {
        let header = jsonwebtoken::decode_header(&token)?;

        let info = DecodeInfo {
            token: &token,
            header: &header,
            validation: &self.validation,
        };

        self.decoder.decode(&info).await
    }
}

pub struct DecodeInfo<'a> {
    pub token: &'a str,
    pub header: &'a Header,
    pub validation: &'a Validation,
}

#[async_trait(?Send)]
pub trait Decoder {
    async fn decode<T>(&self, dec_info: &DecodeInfo<'_>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned;
}

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
