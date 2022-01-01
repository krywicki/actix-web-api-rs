use actix_web::web;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use crate::MongoCollection;

///
/// User Model
///

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub last_login: String,
}

impl MongoCollection for User {
    fn collection_name() -> &'static str {
        "users"
    }

    fn collection<T>(db: &web::Data<Database>) -> Collection<T> {
        db.collection(Self::collection_name())
    }
}
