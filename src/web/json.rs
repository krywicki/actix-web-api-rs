use std::{
    error::Error,
    ops::{self, Deref},
    sync::Arc,
};

use actix_web::web::Json as ActixJson;
use actix_web::FromRequest;
use actix_web::{dev::JsonBody, error::JsonPayloadError};
use actix_web::{http::StatusCode, Error, HttpRequest};

use futures::future::{err, ok, Ready};
use serde::{
    de::{self, DeserializeOwned},
    Serialize,
};
use serde_json;
use validator::Validate;

use crate::{error::RequestError, ErrorCode};

type JsonErrorHandler = Option<Arc<dyn Fn(JsonPayloadError, &HttpRequest) -> dyn Error + Send + Sync>>;

pub struct JsonExtractFut<T> {
    req: Option<HttpRequest>,
    fut: JsonBody<T>,
    err_handler: JsonErrorHandler,
}

pub struct Json<T>(pub T);

impl<T> ops::Deref for Json<T>
where
    T: Validate,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Serialize> Serialize for Json<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> FromRequest for Json<T>
where
    T: DeserializeOwned + Validate,
{
    type Error = RequestError;
    type Future = JsonExtractFut<T>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let config = JsonConfig::from_req(req);

        let limit = config.limit;
        let ctype_required = config.content_type_required;
        let ctype_fn = config.content_type.as_deref();
        let err_handler = config.err_handler.clone();

        JsonExtractFut {
            req: Some(req.clone()),
            fut: JsonBody::new(req, payload, ctype_fn, ctype_required).limit(limit),
            err_handler,
        }
    }
}
