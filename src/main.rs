mod entities;
mod errors;
mod handlers;
mod models;
mod services;
mod utils;

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
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

    let private_key = cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    private_key.clone(),
                )
                .build(),
            )
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(login_routes())
                    .service(book_routes()),
            )
    })
    .bind(("127.0.0.1", 7707))?
    .run()
    .await
}
