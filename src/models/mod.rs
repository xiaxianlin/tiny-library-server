use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

// use crate::entities::book;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomResult<T> {
    pub status: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchParams {
    pub keyword: Option<String>,
    pub page_size: Option<u64>,
    pub current_page: Option<u64>,
    pub order_field: Option<String>,
    pub order_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
