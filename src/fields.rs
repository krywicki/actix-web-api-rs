use std::collections::HashMap;

use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use serde_json::json;
use validator::{self, Validate, ValidateArgs, ValidationError, ValidationErrors};

use crate::{MongoDBFilter, RequestError, TryFromPath};

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

impl FromPath<&String> for EmailOrObjectId {
    fn from_path(name: &'static str, value: &String) -> Result<Self, RequestError> {
        //== attempt ObjectId parse
        if let Ok(value) = ObjectId::parse_str(&value) {
            return Ok(EmailOrObjectId::ObjectId(value));
        }

        if !validator::validate_email(&value) {
            let e = ValidationError {
                code: "INVALID_PATH_PART",
                message: Some("Invalid Email or ObjectId value"),
                params: HashMap::from([(
                    "loc",
                    json!({
                        "path":
                    }),
                )]),
            };
        }

        //== set as email and validate
        let result = EmailOrObjectId::Email(value.clone());

        result.validate_args(&name)?;

        return Ok(result);
    }
}

impl<'v_a> ValidateArgs<'v_a> for EmailOrObjectId {
    type Args = &'static str;

    fn validate_args(&self, field_name: Self::Args) -> Result<(), ValidationErrors> {
        let mut errs = ValidationErrors::new();

        match self {
            Self::Email(ref value) => {
                if !validator::validate_email(value) {
                    errs.add(
                        field_name,
                        ValidationError::new("invalid email or bson Object Id"),
                    )
                }
            }
            _ => {}
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
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
