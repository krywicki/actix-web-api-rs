use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use mongodb::Client;

#[allow(dead_code)]
extern crate api;

use api::endpoints as ep;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mongo = web::Data::new(
        Client::with_uri_str("mongodb://admin:admin@127.0.0.1:27017")
            .await
            .expect("can't connect to database")
            .database("production"),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(mongo.clone())
            .route("/users", web::get().to(ep::users::get_users))
            .route("/users/{id}", web::get().to(ep::users::get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
