use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

use crate::models::CustomResult;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json(CustomResult::<String> {
                    status: 401,
                    message: String::from("未登录"),
                    data: None,
                })
            }
        }
    }
}
