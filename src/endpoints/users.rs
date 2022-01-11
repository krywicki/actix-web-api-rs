use actix_web::{http::StatusCode, web, Responder};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::{
    fields::{EmailOrObjectId, FromPath},
    models::User,
    schemas::{Page, PageBuilder},
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
    let cursor = User::collection(&db)
        .find(None, query.mongo_find_options()?)
        .await?;

    //== build page of results and return
    let page: Page<User> = PageBuilder::from(&query.page_params).build(cursor).await?;
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
        .find_one(id.mongo_filter()?, None)
        .await?;

    //== unwrap and return user
    let user: User = user.ok_or_else(errs::user_not_found)?;
    Ok(web::Json(user))
}

///
/// Update Single User
///
pub async fn update_user(
    id: web::Path<String>,
    db: web::Data<Database>,
    body: web::Json<body::UpdateUserBody>,
) -> RequestResult<impl Responder> {
    let id = EmailOrObjectId::from_path(":id", &*id)?;

    let user = User::collection(&db)
        .find_one_and_update(
            id.mongo_filter()?,
            body.mongo_update_modifications()?,
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        )
        .await?;

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
    use super::*;
    use mongodb::options::FindOptions;
    use validator::Validate;

    use crate::{fields::SortFields, schemas::PageParams, sortfields};

    #[derive(Serialize, Deserialize, Validate)]
    pub struct GetUsersParams {
        pub o: Option<String>,

        #[serde(flatten)]
        pub page_params: PageParams,
    }

    impl GetUsersParams {
        pub fn mongo_find_options(&self) -> Result<Option<FindOptions>, RequestError> {
            let sort = if let Some(ref _sort) = self.o {
                let sort_fields = sortfields!["last_name", "first_name", "email"];
                Some(sort_fields.sort_options(_sort.as_str())?)
            } else {
                None
            };

            Ok(Some(
                FindOptions::builder()
                    .limit(self.page_params.limit)
                    .sort(sort)
                    .build(),
            ))
        }

        fn mongo_filter(&self) -> Result<Option<mongodb::bson::Document>, RequestError> {
            Ok(None)
        }
    }
}

mod body {
    use super::*;
    use mongodb::options::UpdateModifications;
    use validator::Validate;

    use crate::{error::ErrorCode, validators};

    #[derive(Serialize, Deserialize, Validate)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateUserBody {
        #[validate(custom = "validators::validate_alpha_numeric")]
        first_name: Option<String>,

        #[validate(custom = "validators::validate_alpha_numeric")]
        last_name: Option<String>,

        #[validate(custom = "validators::validate_iso_8601")]
        last_login: Option<String>,
    }

    impl UpdateUserBody {
        pub fn mongo_update_modifications(&self) -> Result<UpdateModifications, RequestError> {
            let mut doc = doc! {};

            if let Some(ref first_name) = self.first_name {
                doc.insert("first_name", first_name);
            }

            if let Some(ref last_name) = self.last_name {
                doc.insert("last_name", last_name);
            }

            if let Some(ref last_login) = self.last_login {
                doc.insert("last_login", last_login);
            }

            if doc.is_empty() {
                Err(RequestError::builder()
                    .error(ErrorCode::InvalidBody)
                    .message("Cannot update user with null/empty content")
                    .build())
            } else {
                Ok(UpdateModifications::Document(doc! { "$set": doc }))
            }
        }
    }
}
