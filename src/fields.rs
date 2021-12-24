use std::{borrow::Borrow, collections::HashMap};

use actix_web::http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use serde_json::json;
use validator::{self, Validate, ValidateArgs, ValidationError, ValidationErrors};

use crate::{error::ErrorCode, MongoDBFilter, RequestError, TryFromPath};

pub trait FromPath<T>: Sized
where
    T: Sized,
{
    fn from_path(name: &'static str, value: T) -> Result<Self, RequestError>;
}

///
/// EmailOrObjectId
///

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmailOrObjectId {
    Email(String),
    ObjectId(ObjectId),
}

impl EmailOrObjectId {
    fn validate(value: &String) -> Result<Self, String> {
        //== attempt ObjectId parse
        if let Ok(value) = ObjectId::parse_str(&value) {
            return Ok(EmailOrObjectId::ObjectId(value));
        }

        if !validator::validate_email(value.as_str()) {
            return Err("Invalid email or objectId value".into());
        }

        //== set as email and validate
        return Ok(EmailOrObjectId::Email(value.clone()));
    }
}

impl FromPath<&String> for EmailOrObjectId {
    fn from_path(name: &'static str, value: &String) -> Result<Self, RequestError> {
        Ok(EmailOrObjectId::validate(&value).map_err(|e| {
            RequestError::builder()
                .code(StatusCode::BAD_REQUEST)
                .error(ErrorCode::InvalidPathPart)
                .message(e)
                .build()
        })?)
    }
}

impl MongoDBFilter for EmailOrObjectId {
    fn mongo_filter(&self) -> Document {
        match self {
            Self::Email(ref value) => doc! { "email": value },
            Self::ObjectId(ref value) => doc! { "_id": value },
        }
    }
}
