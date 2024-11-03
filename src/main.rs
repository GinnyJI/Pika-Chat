mod routes;
mod middleware;
mod models;
mod config;

use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use middleware::auth_middleware::AuthMiddleware;
use routes::{test_protected_route, logout_user};  // Import the test route function

// Macro to mark the main function as an Actix Web entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging with env_logger for runtime log output
    env_logger::init();

    // Load environment variables from a .env file, if present
    dotenvy::dotenv().ok();

    // Establish a connection pool to the SQLite database using SQLx
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    // Configure and run the Actix Web HTTP server
    HttpServer::new(move || {
        App::new()
            // Register the database pool as application data, making it accessible to route handlers
            .app_data(web::Data::new(pool.clone()))

            // Register public routes that don't require authentication
            .service(routes::register_user)
            .service(routes::login_user)
            // Register the logout route with AuthMiddleware to protect it
            .service(
                web::resource("/logout")
                    .wrap(AuthMiddleware)
                    .route(web::post().to(logout_user))
            )

            // Register the test route with AuthMiddleware for testing
            .service(
                web::resource("/test-protected")
                    .wrap(AuthMiddleware)
                    .route(web::get().to(test_protected_route))
            )
            // Register protected routes that require JWT authentication
            // .service(
            //     // Apply AuthMiddleware to protect the /rooms route
            //     // Route for creating a new chat room
            //     web::resource("/rooms")
            //         .wrap(AuthMiddleware)
            //         .route(web::post().to(routes::create_room))
            // )
            // .service(
            //     // Route for joining a chat room
            //     web::resource("/rooms/{room_id}/join")
            //         .wrap(AuthMiddleware)
            //         .route(web::post().to(routes::join_room))
            // )
            // .service(
            //     // Route for establishing a WebSocket connection
            //     web::resource("/ws/rooms/{room_id}")
            //         .wrap(AuthMiddleware)
            //         .route(web::get().to(routes::ws_room)) 
            // )
    })
    // Bind the server to the address and port
    .bind(("127.0.0.1", 8080))?
    // Run the server and await its completion
    .run()
    .await
}
