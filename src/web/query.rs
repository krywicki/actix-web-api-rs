use std::{error::Error, ops::Deref};

use actix_web::http::StatusCode;
use actix_web::FromRequest;
use futures::future::{err, ok, Ready};
use serde::de;
use serde_json;
use validator::Validate;

use crate::{error::RequestError, ErrorCode};

pub struct Query<T: Validate>(pub T);

impl<T> Deref for Query<T>
where
    T: Validate,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for Query<T>
where
    T: de::DeserializeOwned + Validate,
{
    type Error = RequestError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        serde_urlencoded::from_str::<T>(req.query_string())
            .map_err(RequestError::from)
            .and_then(|q| q.validate().map(move |_| q).map_err(RequestError::from))
            .map_err(RequestError::from)
            .map(|value| ok(Query(value)))
            .unwrap_or_else(|e| err(e))
    }
}

impl From<serde_urlencoded::de::Error> for RequestError {
    fn from(error: serde_urlencoded::de::Error) -> Self {
        RequestError::builder()
            .code(StatusCode::INTERNAL_SERVER_ERROR)
            .error(ErrorCode::InternalServerError)
            .message("URL failed to decode")
            .detail(Some(error.to_string().into()))
            .source(match error.source() {
                Some(source) => Some(serde_json::Value::String(source.to_string())),
                None => None,
            })
            .build()
    }
}
