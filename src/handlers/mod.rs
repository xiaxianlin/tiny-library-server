use crate::{
    errors::ServiceError,
    models::{CustomResult, LoginData},
};
use actix_session::SessionExt;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, HttpResponse};
use std::future::{ready, Ready};

pub mod auth;
pub mod book;

pub struct LoginAuth;

impl FromRequest for LoginAuth {
    type Error = Error;
    type Future = Ready<Result<LoginAuth, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = req.get_session();
        if let Some(value) = req.head().headers().get("x-xxl-token") {
            let key = String::from_utf8(value.as_bytes().to_vec()).unwrap();
            let data: Option<LoginData> = session.get::<LoginData>(&key).unwrap();
            if data.is_some() {
                return ready(Ok(LoginAuth {}));
            }
        }

        ready(Err(ServiceError::Unauthorized.into()))
    }
}

pub fn success<T>(status: i32, message: &str, data: Option<T>) -> HttpResponse
where
    T: serde::Serialize,
{
    HttpResponse::Ok().json(CustomResult {
        status: status,
        message: String::from(message),
        data,
    })
}
