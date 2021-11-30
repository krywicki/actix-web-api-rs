use actix_web::Responder;

pub mod endpoints;
pub mod models;
pub mod utils;

mod error;
pub use error::RequestError;
pub use error::RequestErrorBuilder;

pub type RequestResult<T: Responder> = std::result::Result<T, RequestError>;
