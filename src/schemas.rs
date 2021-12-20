use serde::{Deserialize, Serialize};

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
    pub next: Option<String>,
}
