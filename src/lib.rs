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
use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::Database;

pub type RequestResult<T> = std::result::Result<T, RequestError>;

pub trait MongoDB {
    fn collection_name() -> &'static str;
    fn collection<T: Sized>(db: &actix_web::web::Data<Database>) -> Collection<T>;
}

pub trait MongoDBFilter {
    fn mongo_filter(&self) -> Document;
}

pub trait TryFromPath<T> {
    fn try_from_path(name: &str, value: &actix_web::web::Path<T>) -> Result<Self, RequestError>
    where
        Self: Sized;
}
