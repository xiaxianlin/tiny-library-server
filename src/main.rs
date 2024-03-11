mod entities;
mod errors;
mod handlers;
mod models;
mod services;
mod utils;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie, middleware, web, App, HttpServer, Scope};
use handlers::{auth, book};
use models::AppState;

fn login_routes() -> Scope {
    web::scope("/auth")
        .service(auth::login)
        .service(auth::logout)
}

fn book_routes() -> Scope {
    web::scope("/book")
        .service(book::create)
        .service(book::update)
        .service(book::delete)
        .service(book::update_cover)
        .service(book::replace_cover)
        .service(book::search)
        .service(book::find_by_id)
        .service(book::find_by_isbn)
        .service(book::find_by_douban)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let conn = utils::get_db_connection()
        .await
        .expect("Database connect error.");

    let state = AppState { conn };

    let secret_key = cookie::Key::derive_from("scg4WX1d2YMpzFqO0y8oAr72NNl6AIn5".as_bytes());
    let store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(SessionMiddleware::builder(store.clone(), secret_key.clone()).build())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(login_routes())
                    .service(book_routes()),
            )
    })
    .bind(("0.0.0.0", 7707))?
    .run()
    .await
}
