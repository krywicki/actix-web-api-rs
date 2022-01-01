use actix_web;

pub mod endpoints;
pub mod fields;
pub mod models;
pub mod schemas;
pub mod utils;
pub mod validators;
pub mod web;

mod error;
pub use error::RequestError;
pub use error::RequestErrorBuilder;
use mongodb::{bson::Document, options::FindOptions, Collection, Database};

pub type RequestResult<T> = std::result::Result<T, RequestError>;

pub trait MongoCollection {
    fn collection_name() -> &'static str;
    fn collection<T: Sized>(db: &actix_web::web::Data<Database>) -> Collection<T>;
}

pub trait MongoFilter {
    fn mongo_filter(&self) -> Option<Document>;
}

pub trait MongoFindOptions {
    fn mongo_find_options(&self) -> Option<FindOptions>;
}

pub trait MongoTryFindOptions {
    type Error;

    fn mongo_try_find_options(&self) -> Result<Option<FindOptions>, Self::Error>;
}

pub trait MongoTryFilter {
    type Error;

    fn mongo_try_filter(&self) -> Result<Option<Document>, Self::Error>;
}
