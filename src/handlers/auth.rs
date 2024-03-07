use crate::{handlers::success, models::LoginData, utils::get_login_info};
use actix_session::Session;
use actix_web::{post, web::Json, HttpRequest, HttpResponse};

fn check(req: HttpRequest, session: &Session) -> Option<String> {
    match req.head().headers().get("x-xxl-token") {
        Some(value) => {
            let key = String::from_utf8(value.as_bytes().to_vec()).unwrap();
            let data = session.get::<LoginData>(&key).unwrap();
            if let Some(_) = data {
                Some(key)
            } else {
                None
            }
        }
        None => None,
    }
}

#[post("/login")]
pub async fn login(req: HttpRequest, session: Session, info: Json<LoginData>) -> HttpResponse {
    if let Some(key) = check(req, &session) {
        return success(0, "已登录", Some(key));
    }
    let data: LoginData = info.into_inner();
    let (username, password) = get_login_info();
    if data.username.eq(&username) && data.password.eq(&password) {
        let user_string = serde_json::to_string(&data).unwrap();
        let key = format!("{:x}", md5::compute(user_string));
        session.insert(&key, data).unwrap();
        success(0, "", Some(key))
    } else {
        success::<String>(-1, "用户名或密码错误", None)
    }
}

#[post("/logout")]
pub async fn logout(req: HttpRequest, session: Session) -> HttpResponse {
    let res = check(req, &session);
    if res.is_none() {
        return success::<String>(0, "已退出", None);
    }
    session.purge();
    success::<String>(0, "退出成功", None)
}
