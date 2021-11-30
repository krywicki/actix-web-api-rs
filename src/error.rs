use std::{error::Error, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;

#[derive(Debug)]
pub struct RequestError {
    pub code: StatusCode,
    pub message: String,
    pub detail: Option<serde_json::Value>,
    pub source: Option<Box<dyn Error>>,
}

pub struct RequestErrorBuilder {
    code: StatusCode,
    message: String,
    detail: Option<serde_json::Value>,
    source: Option<Box<dyn Error>>,
}

impl Default for RequestErrorBuilder {
    fn default() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "".into(),
            detail: None,
            source: None,
        }
    }
}

impl RequestErrorBuilder {
    pub fn code(&mut self, code: StatusCode) -> &mut Self {
        self.code = code;
        self
    }

    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }

    pub fn detail(&mut self, detail: serde_json::Value) -> &mut Self {
        self.detail = Some(detail);
        self
    }

    pub fn source(&mut self, source: Box<dyn std::error::Error>) -> &mut self {
        self.source = Some(source);
        self
    }

    pub fn build(self) -> RequestError {
        RequestError {}
    }
}

impl Error for RequestError {}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = json!({
            "message":self.message,
            "detail":self.detail,
            "source": match self.source {
                Some(ref err) => err.to_string(),
                None => "".into()
            }
        })
        .to_string();

        write!(f, "{}", json)
    }
}

impl ResponseError for RequestError {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        let json = json!({
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
            message: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            detail: None,
            source: Some(Box::new(error)),
        }
    }
}
