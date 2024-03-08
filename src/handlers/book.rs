use crate::{
    entities::prelude::*,
    handlers::success,
    models::SearchParams,
    services::{book, douban},
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{Bytes, Data, Json, Path, Query},
    HttpResponse,
};
use serde_json::{json, Value};

use super::LoginAuth;

#[get("/douban/{isbn}")]
pub async fn find_by_douban(
    _: LoginAuth,
    state: Data<AppState>,
    info: Path<String>,
) -> HttpResponse {
    let info = info.into_inner();
    let data = book::find_by_isbn(&state.conn, info.clone()).await;
    if data.is_none() {
        let data = douban::get_book_by_douban(info).await;
        return success(0, "", data);
    }
    success(0, "", data)
}

#[get("/{id}")]
pub async fn find_by_id(_: LoginAuth, state: Data<AppState>, id: Path<i64>) -> HttpResponse {
    let data = book::find_by_id(&state.conn, id.into_inner()).await;
    success(0, "", data)
}

#[get("/isbn/{isbn}")]
pub async fn find_by_isbn(_: LoginAuth, state: Data<AppState>, isbn: Path<String>) -> HttpResponse {
    let data = book::find_by_isbn(&state.conn, isbn.into_inner()).await;
    success(0, "", data)
}

#[get("/search")]
pub async fn search(
    _: LoginAuth,
    state: Data<AppState>,
    params: Query<SearchParams>,
) -> HttpResponse {
    let params = params.into_inner();
    let data = book::search(&state.conn, params.clone()).await;
    let count = book::count(&state.conn, params).await;
    success(0, "", Some(json!({"list":data, "total":count})))
}

#[post("/add")]
pub async fn create(_: LoginAuth, state: Data<AppState>, info: Json<BookModel>) -> HttpResponse {
    let id = book::create(&state.conn, info.into_inner()).await;
    success(0, "", Some(id))
}

#[delete("/{id}")]
pub async fn delete(_: LoginAuth, state: Data<AppState>, id: Path<i64>) -> HttpResponse {
    let rows = book::delete(&state.conn, id.into_inner()).await;
    success(0, "", Some(rows))
}

#[put("/{id}")]
pub async fn update(
    _: LoginAuth,
    state: Data<AppState>,
    id: Path<i64>,
    info: Json<Value>,
) -> HttpResponse {
    let rows = book::update(&state.conn, id.into_inner(), info.into_inner()).await;
    success(0, "", Some(rows))
}

#[post("/cover/{id}")]
pub async fn update_cover(
    _: LoginAuth,
    state: Data<AppState>,
    id: Path<i64>,
    body: Bytes,
) -> HttpResponse {
    let res = book::update_cover(&state.conn, id.into_inner(), body.to_vec()).await;
    success(0, "", Some(res))
}

#[put("/cover/{id}")]
pub async fn replace_cover(_: LoginAuth, state: Data<AppState>, id: Path<i64>) -> HttpResponse {
    let res = book::replace_cover(&state.conn, id.into_inner()).await;
    success(0, "", Some(res))
}
