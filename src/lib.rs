use actix_web;

pub mod endpoints;
pub mod fields;
pub mod models;
pub mod schemas;
pub mod utils;
pub mod validators;
pub mod web;

mod error;
pub use error::{ErrorCode, RequestError, RequestErrorBuilder};

use mongodb::{bson::Document, Collection, Database};

pub type RequestResult<T> = std::result::Result<T, RequestError>;

pub trait MongoCollection {
    fn collection_name() -> &'static str;
    fn collection<T: Sized>(db: &actix_web::web::Data<Database>) -> Collection<T>;
}

pub trait MongoFilter {
    type Error;

    fn mongo_filter(&self) -> Result<Document, Self::Error>;
}

pub trait MongoOptionalFilter {
    type Error;

    fn mongo_filter(&self) -> Result<Option<Document>, Self::Error>;
}
