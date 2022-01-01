use actix_web::{http::StatusCode, web, Responder};
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

use crate::{
    fields::{EmailOrObjectId, FromPath},
    models::User,
    schemas::{Page, PageBuilder, UserOut},
    web::Query,
    MongoCollection, MongoFilter, RequestError, RequestResult,
};

///
/// Get List of Users
///
pub async fn get_users(
    query: Query<qparams::GetUsersParams>,
    db: web::Data<Database>,
) -> RequestResult<impl Responder> {
    //== create collection cursor
    let cursor = User::collection(&db).find(None, None).await?;

    //== build page of results and return
    let page: Page<User> = PageBuilder::default().build(cursor).await?;
    Ok(web::Json(page))
}

///
/// Get Single User
///
pub async fn get_user(
    id: web::Path<String>,
    db: web::Data<Database>,
) -> RequestResult<impl Responder> {
    let id = EmailOrObjectId::from_path(":id", id.as_ref())?;

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
            .message("User not found")
            .build()
    }
}

mod qparams {
    use mongodb::options::FindOptions;
    use validator::Validate;

    use crate::{fields::SortFields, sortfields, MongoFindOptions, MongoTryFindOptions};

    use super::*;

    #[derive(Serialize, Deserialize, Validate)]
    pub struct GetUsersParams {
        o: Option<String>,

        #[serde(default = "GetUsersParams::default_limit")]
        #[validate(range(min = 1, max = 1000))]
        limit: usize,
    }

    impl GetUsersParams {
        pub fn default_limit() -> usize {
            100
        }
    }

    impl MongoFilter for GetUsersParams {
        fn mongo_filter(&self) -> Option<mongodb::bson::Document> {
            None
        }
    }

    impl MongoTryFindOptions for GetUsersParams {
        type Error = RequestError;

        fn mongo_try_find_options(&self) -> Result<Option<FindOptions>, Self::Error> {
            let sort_fields: SortFields = sortfields!["last_name", "first_name", "email"];

            let sort = if let Some(ref _sort) = self.o {
                Some(sort_fields.sort_options(_sort.as_str())?)
            } else {
                None
            };

            Ok(Some(FindOptions::builder().limit(10).sort(sort).build()))
        }
    }
}
