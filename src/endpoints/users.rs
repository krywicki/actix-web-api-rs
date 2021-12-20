use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    fields::{EmailOrObjectId, FromField},
    models::User,
    schemas::{Page, UserOut},
    validators,
    web::Query,
    MongoDB, MongoDBFilter, RequestError, RequestResult,
};

pub async fn get_users(query: Query<qparams::GetUsersParams>) -> RequestResult<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub async fn get_user(
    id: web::Path<String>,
    db: web::Data<Database>,
) -> RequestResult<impl Responder> {
    let id = EmailOrObjectId::from_field("id", id.as_ref())?;

    //== get user
    let user = User::collection(&db)
        .find_one(id.mongo_filter(), None)
        .await?;

    //== unwrap and return user
    let user: User = user.ok_or_else(errs::user_not_found)?;
    Ok(web::Json(user))
}

mod errs {
    use super::*;

    pub fn user_not_found() -> RequestError {
        RequestError::builder()
            .code(StatusCode::NOT_FOUND)
            .message("User not found".into())
            .build()
    }
}

mod qparams {
    use validator::Validate;

    use super::*;

    #[derive(Serialize, Deserialize, Validate)]
    pub struct GetUsersParams {
        o: Option<String>,
    }
}
