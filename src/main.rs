use actix_web::{http::StatusCode, middleware::Logger, web, App, HttpServer};
use log::LevelFilter;
use mongodb::Client;

#[allow(dead_code)]
extern crate api;

use api::{endpoints as ep, ErrorCode, RequestError};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let mongo = web::Data::new(
        Client::with_uri_str("mongodb://admin:admin@127.0.0.1:27017")
            .await
            .expect("can't connect to database")
            .database("production"),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%s - %r"))
            .app_data(mongo.clone())
            .app_data(json_config())
            .route("/users", web::get().to(ep::users::get_users))
            .route("/users/{id}", web::get().to(ep::users::get_user))
            .route("/users/{id}", web::patch().to(ep::users::update_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .content_type(|mime| {
            (mime.type_() == "text" && mime.subtype() == "plain")
                || (mime.type_() == "application" && mime.subtype() == "json")
        })
        .error_handler(|err, _req| {
            RequestError::builder()
                .code(StatusCode::BAD_REQUEST)
                .error(ErrorCode::InvalidBody)
                .message("Invalid Json Content")
                .detail(Some(err.to_string().into()))
                .build()
                .into()
        })
}
