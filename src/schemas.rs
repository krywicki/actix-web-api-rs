use futures::TryStreamExt;
use mongodb::Cursor;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
    pub next: Option<usize>,
}

impl<T: Sized> Page<T> {
    pub fn new() -> Self {
        Self {
            count: 0,
            items: vec![],
            next: None,
        }
    }

    pub fn builder() -> PageBuilder {
        PageBuilder::default()
    }
}

pub struct PageBuilder {
    pub limit: usize,
    pub offset: usize,
}

impl Default for PageBuilder {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
        }
    }
}

impl PageBuilder {
    pub fn limit(mut self, value: usize) -> Self {
        self.limit = value;
        self
    }

    pub fn from(mut self, value: usize) -> Self {
        self.offset = value;
        self
    }

    pub async fn build<T>(self, mut cursor: Cursor<T>) -> Result<Page<T>, RequestError>
    where
        T: DeserializeOwned + Sync + Unpin + Send,
    {
        let mut page = Page::new();

        // get items
        while page.items.len() < self.limit {
            if let Some(item) = cursor.try_next().await.map_err(RequestError::from)? {
                page.items.push(item);
            } else {
                break;
            }
        }

        // set items count
        page.count = page.items.len();

        // set next (if any)
        if page.count == self.limit {
            page.next = Some(page.count + self.offset);
        }

        Ok(page)
    }
}
