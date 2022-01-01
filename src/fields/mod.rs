use crate::RequestError;

mod common;
mod sort_fields;

pub use common::EmailOrObjectId;
pub use sort_fields::{SortField, SortFields};

pub trait FromPath<T>: Sized
where
    T: Sized,
{
    fn from_path(name: &'static str, value: T) -> Result<Self, RequestError>;
}
