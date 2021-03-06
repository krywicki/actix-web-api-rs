use std::{borrow::Cow, error::Error, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use lazy_static::__Deref;
use log::{error, warn};
use serde_json::{json, Map};
use validator::{ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct RequestError {
    pub code: StatusCode,
    pub error: String,
    pub message: String,
    pub detail: Option<serde_json::Value>,
    pub source: Option<serde_json::Value>,
}

impl RequestError {
    pub fn builder() -> RequestErrorBuilder {
        RequestErrorBuilder::default()
    }
}

pub struct RequestErrorBuilder {
    code: StatusCode,
    error: String,
    message: String,
    detail: Option<serde_json::Value>,
    source: Option<serde_json::Value>,
}

impl Default for RequestErrorBuilder {
    fn default() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            error: "".into(),
            message: "".into(),
            detail: None,
            source: None,
        }
    }
}

impl RequestErrorBuilder {
    pub fn code(mut self, code: StatusCode) -> Self {
        self.code = code;
        self
    }

    pub fn error<T: Into<String>>(mut self, error: T) -> Self {
        self.error = error.into();
        self
    }

    pub fn message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = message.into();
        self
    }

    pub fn detail<T>(mut self, detail: T) -> Self
    where
        T: Into<Option<serde_json::Value>>,
    {
        self.detail = detail.into();
        self
    }

    pub fn source(mut self, source: Option<serde_json::Value>) -> Self {
        self.source = source;
        self
    }

    pub fn build(self) -> RequestError {
        RequestError {
            code: self.code,
            error: self.error,
            message: self.message,
            detail: self.detail,
            source: self.source,
        }
    }
}

impl Error for RequestError {}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = json!({
            "code": self.code.as_u16().to_string(),
            "error": self.error,
            "message":self.message,
            "detail":self.detail,
            "source": self.source
        });

        write!(f, "\n{}", serde_json::to_string_pretty(&json).unwrap())
    }
}

impl ResponseError for RequestError {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        if self.status_code().is_client_error() {
            warn!("{}", self.to_string());
        } else if self.status_code().is_server_error() {
            error!("{}", self.to_string());
        }

        let json = json!({
            "error": self.error,
            "message": self.message,
            "detail": self.detail
        });

        HttpResponse::build(self.status_code()).json(json)
    }
}

impl From<mongodb::error::Error> for RequestError {
    fn from(error: mongodb::error::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            error: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            message: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            detail: None,
            source: if let Some(source) = error.source() {
                Some(serde_json::Value::String(source.to_string()))
            } else {
                None
            },
        }
    }
}

//
// Convert Validation Errors into RequestError
//
impl From<ValidationError> for RequestError {
    fn from(error: ValidationError) -> Self {
        RequestError {
            code: StatusCode::BAD_REQUEST,
            error: ErrorCode::ValidationError.into(),
            message: error
                .message
                .unwrap_or(Cow::from("Validation Error"))
                .into(),
            detail: Some(serde_json::value::to_value(error.params).unwrap()),
            source: None,
        }
    }
}

impl From<ValidationErrors> for RequestError {
    fn from(error: ValidationErrors) -> Self {
        let mut errors: Map<String, serde_json::Value> = Map::new();

        error.errors().iter().for_each(|e| {
            errors.insert(e.0.to_string(), serde_json::to_value(e.1).unwrap());
        });

        RequestError {
            code: StatusCode::BAD_REQUEST,
            error: ErrorCode::ValidationError.into(),
            message: "Validation Error".into(),
            detail: Some(errors.into()),
            source: None,
        }
    }
}

pub enum ErrorCode {
    InvalidPathPart,
    InvalidQueryParam,
    ResourceNotFound,
    ValidationError,
    InvalidBody,
    InternalServerError,
}

impl Into<String> for ErrorCode {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match *self {
            Self::InvalidPathPart => "INVALID_PATH_PART",
            Self::ResourceNotFound => "RESOURCE_NOT_FOUND",
            Self::InvalidQueryParam => "INVALID_QUERY_PARAM",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::InvalidBody => "INVALID_BODY",
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
        };

        write!(f, "{}", val)
    }
}
