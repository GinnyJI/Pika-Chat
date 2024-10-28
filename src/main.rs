mod routes; // Import the routes module

use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging with env_logger
    env_logger::init();

    dotenvy::dotenv().ok();
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::register_user)
            .service(routes::login_user)
            .service(routes::logout_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
