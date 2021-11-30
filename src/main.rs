use actix_web::{web, App, HttpServer};
use mongodb::Client;

#[allow(dead_code)]
extern crate api;

use api::endpoints as ep;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mongo = Client::with_uri_str("mongodb://admin:admin@127.0.0.1:27017")
        .await
        .expect("can't connect to database");

    HttpServer::new(move || {
        App::new()
            .data(web::Data::new(mongo.database("production")))
            .route("/users", web::get().to(ep::users::get_users))
            .route("/users/{id}", web::get().to(ep::users::get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
