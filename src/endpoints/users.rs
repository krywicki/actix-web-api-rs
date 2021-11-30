use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use mongodb::{bson::doc, Database};

use crate::{
    models::{MongoDB, User},
    RequestError, RequestErrorBuilder, RequestResult,
};

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_user(
    id: web::Path<String>,
    db: web::Data<Database>,
) -> RequestResult<impl Responder> {
    let user = User::collection(&db)
        .find_one(doc! {"email": &*id}, None)
        .await?;

    let user = user.ok_or_else(|| {
        let err = RequestErrorBuilder::new()
            .code(StatusCode::NOT_FOUND)
            .build();

        return err;
    })?;

    Ok(HttpResponse::Ok())
}
