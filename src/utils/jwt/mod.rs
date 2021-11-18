use async_trait::async_trait;
use jsonwebtoken::{errors::Error, Header, TokenData, Validation};
use serde::de::DeserializeOwned;

mod jwks_decoder;

pub mod decoders {
    pub use super::jwks_decoder::JwksDecoder;
}

///
/// JWT Decoder
///

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

///
/// Decode Info
///

pub struct DecodeInfo<'a> {
    pub token: &'a str,
    pub header: &'a Header,
    pub validation: &'a Validation,
}

///
/// Decoder trait
///

#[async_trait(?Send)]
pub trait Decoder {
    async fn decode<T>(&self, dec_info: &DecodeInfo<'_>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned;
}
