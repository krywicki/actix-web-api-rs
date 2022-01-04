use futures::TryStreamExt;
use mongodb::Cursor;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;

use crate::RequestError;

///
/// UserOut Schema
///
#[derive(Debug, Serialize, Deserialize)]
pub struct UserOut {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub last_login: String,
}

///
/// Page (Pagination) Schema
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T: Sized> {
    pub count: usize,
    pub items: Vec<T>,
    pub next: Option<i64>,
}

impl<T: Sized> Page<T> {
    pub fn new() -> Self {
        Self {
            count: 0,
            items: vec![],
            next: None,
        }
    }
}

pub struct PageBuilder {
    pub offset: i64,
    pub limit: i64,
}

impl PageBuilder {
    pub async fn build<T>(self, cursor: Cursor<T>) -> Result<Page<T>, RequestError>
    where
        T: DeserializeOwned + Sync + Unpin + Send,
    {
        let items: Vec<T> = cursor.try_collect().await?;
        let next = if items.len() < self.limit as usize {
            None
        } else {
            Some(self.offset + self.limit)
        };

        Ok(Page {
            count: items.len(),
            next: next,
            items: items,
        })
    }
}

impl From<&PageParams> for PageBuilder {
    fn from(params: &PageParams) -> Self {
        Self {
            limit: params.limit,
            offset: params.offset,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PageParams {
    #[validate(range(min = 1, max = 1000))]
    #[serde(default = "PageParams::default_limit")]
    pub limit: i64,

    #[validate(range(min = 0))]
    #[serde(default = "PageParams::default_offset")]
    pub offset: i64,
}

impl PageParams {
    pub fn default_limit() -> i64 {
        100
    }
    pub fn default_offset() -> i64 {
        0
    }
}
