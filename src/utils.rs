use actix_web::Result;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);

    let mut opt = ConnectOptions::new(database_url);
    opt.sqlx_logging(true);

    let db = Database::connect(opt).await?;
    Ok(db)
}

pub fn get_oss_client() -> aliyun_oss_client::Client {
    dotenv().ok();
    aliyun_oss_client::Client::from_env().unwrap()
}

pub fn get_oss_dir() -> String {
    dotenv().ok();
    std::env::var("ALIYUN_DIR").expect("ALIYUN_DIR must be set")
}

pub fn get_login_info() -> (String, String) {
    dotenv().ok();
    let username = std::env::var("LOGIN_USERNAME").expect("LOGIN_USERNAME must be set");
    let password = std::env::var("LOGIN_PASSWORD").expect("LOGIN_PASSWORD must be set");
    (username, password)
}
