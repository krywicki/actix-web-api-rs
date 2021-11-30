use actix_web::web;
use mongodb::{bson::Document, Collection, Database};
use serde::{Deserialize, Serialize};

pub trait MongoDB {
    fn collection_name() -> &'static str;

    fn collection(db: &web::Data<Database>) -> Collection<Self>
    where
        Self: Sized;

    fn doc_collection(db: &web::Data<Database>) -> Collection<Document> {
        db.collection(Self::collection_name())
    }
}

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

    fn collection(db: &web::Data<Database>) -> Collection<Self> {
        db.collection(Self::collection_name())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserOut {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub last_login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T: Sized> {
    pub count: usize,
    pub items: Vec<T>,
    pub next: Option<String>,
}
