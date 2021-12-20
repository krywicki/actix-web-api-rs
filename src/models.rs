use actix_web::web;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use crate::MongoDB;

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

impl MongoDB for User {
    fn collection_name() -> &'static str {
        "users"
    }

    fn collection<T>(db: &web::Data<Database>) -> Collection<T> {
        db.collection(Self::collection_name())
    }
}
