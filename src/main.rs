mod routes;
mod middleware;
mod models;
mod config;

use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use middleware::auth_middleware::AuthMiddleware;
use routes::auth::{register_user, login_user, logout_user, RegisterData, LoginData};
use routes::room::{create_room, add_room_member, get_rooms, get_room_members, RoomMember, Room, RoomInfo, RoomsResponse};
use routes::test_routes::test_protected_route;
use models::response::{MessageResponse, ErrorResponse, TokenResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::auth::register_user, 
        crate::routes::auth::login_user, 
        crate::routes::auth::logout_user,
        crate::routes::room::get_rooms, 
        crate::routes::room::create_room, 
        crate::routes::room::add_room_member,
        crate::routes::room::get_room_members
    ), 
    components(schemas(RoomMember, Room, RoomInfo, RoomsResponse, RegisterData, LoginData, MessageResponse, TokenResponse, ErrorResponse))
)]
struct ApiDoc;

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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            // Scope all routes under `/api`
            .service(
                web::scope("/api")
                    // Register public routes that don't require authentication
                    .service(register_user)
                    .service(login_user)
                    
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
                    
                    // Register protected routes for chat room management
                    .service(
                        web::resource("/rooms")
                            .wrap(AuthMiddleware)
                            .route(web::post().to(create_room))  // Handle POST requests to create a room
                            .route(web::get().to(get_rooms))     // Handle GET requests to retrieve rooms
                    )
                    .service(
                        web::resource("/rooms/{room_id}/members")
                            .wrap(AuthMiddleware)
                            .route(web::post().to(add_room_member)) // POST to add a member
                            .route(web::get().to(get_room_members)) // GET to retrieve members
                    )
            )
    })
    // Bind the server to the address and port
    .bind(("127.0.0.1", 80))?
    // Run the server and await its completion
    .run()
    .await
}
