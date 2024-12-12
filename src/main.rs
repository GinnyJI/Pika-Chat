mod config;
mod middleware;
mod models;
mod routes;
mod websockets;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use middleware::auth_middleware::AuthMiddleware;
use routes::auth::{register_user, login_user, logout_user, AuthData};
use routes::room::{create_room, add_room_member, get_rooms, get_room_members, join_room_ws, get_user_presence, RoomMember, Room, RoomInfo, RoomsResponse};
use models::response::{ErrorResponse, MessageResponse, TokenResponse};
use routes::test_routes::test_protected_route;
use sqlx::SqlitePool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use websockets::chat_session::RoomServer;
// Allow the ApiDoc struct to serve as a container for OpenAPI documentation
// generated based on the specified paths and components.
#[derive(OpenApi)]
#[openapi(
    // Specify the endpoints (paths) that should be included in the documentation.
    paths(
        crate::routes::auth::register_user,
        crate::routes::auth::login_user,
        crate::routes::auth::logout_user,
        crate::routes::room::get_rooms,
        crate::routes::room::create_room,
        crate::routes::room::add_room_member,
        crate::routes::room::get_room_members,
        crate::routes::room::join_room_ws,
        crate::routes::room::get_user_presence
    ),
    // Define all the schemas (data structures) that will be used in the API documentation.
    components(schemas(RoomMember, Room, RoomInfo, RoomsResponse, AuthData, MessageResponse, TokenResponse, ErrorResponse))
)]
// Empty struct ApiDoc serves as the root for the OpenAPI spec.
// #[openapi(...)] generates a full OpenAPI spec, including all paths and schemas.
// utoipa uses this to consolidate documentation within ApiDoc.
struct ApiDoc;

// Macro to mark the main function as an Actix Web entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging with env_logger for runtime log output
    env_logger::init();

    // Load environment variables from a .env file, if present
    dotenvy::dotenv().ok();

    // Establish a connection pool to the SQLite database using SQLx
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    // Initialize a new instance of RoomServer (managing chat rooms) and start it as an Actor.
    // This actor will handle WebSocket communication for room sessions.
    // Calling start() on RoomServer here starts the actor and calls its `started` method (if implemented),
    // signaling the actor is ready to receive and process messages.
    let room_server = RoomServer::new().start();

    // Configure and run the Actix Web HTTP server
    HttpServer::new(move || {
        // The closure passed to HttpServer::new is used to create a new instance of App
        // for each thread in the server's thread pool. Each thread runs independently,
        // and the App handles incoming HTTP requests assigned to that thread.

        // The `move` keyword ensures that variables from the outer scope (like pool and room_server)
        // are captured by value (ownership is transferred). This is essential because the closure
        // will be executed on multiple threads, and variables captured by reference would
        // lead to unsafe behavior or invalid references.
        App::new()
            // Register the database pool as application data, making it accessible to route handlers
            // Register the database connection pool (pool) as application data.
            // pool.clone() ensures that the same pool is safely shared across multiple threads.
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allow frontend origin
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"]) // Allow specific methods
                    .allow_any_header() // Allow any custom headers if required
                    .supports_credentials() // Support cookies and credentials
                    .max_age(3600), // Cache preflight response for 1 hour
            )
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            // Register the RoomServer actor, shared across threads for managing chat room sessions.
            .app_data(web::Data::new(room_server.clone()))
            .service(
                web::resource("/ws/rooms/{room_id}")
                    .route(web::get().to(join_room_ws)),
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
                            .route(web::post().to(logout_user)),
                    )
                    // Register the test route with AuthMiddleware for testing
                    .service(
                        web::resource("/test-protected")
                            .wrap(AuthMiddleware)
                            .route(web::get().to(test_protected_route)),
                    )
                    // Register protected routes for chat room management
                    .service(
                        web::resource("/rooms")
                            .wrap(AuthMiddleware)
                            .route(web::post().to(create_room)) // Handle POST requests to create a room
                            .route(web::get().to(get_rooms)), // Handle GET requests to retrieve rooms
                    )
                    .service(
                        web::resource("/rooms/{room_id}/members")
                            .wrap(AuthMiddleware)
                            .route(web::post().to(add_room_member)) // POST to add a member
                            .route(web::get().to(get_room_members)), // GET to retrieve members
                    )
                    .service(
                        web::resource("/users/presence/{room_id}")
                            .wrap(AuthMiddleware)
                            .route(web::get().to(get_user_presence)),
                    ),
            )
    })
    // Bind the server to the address and port
    .bind(("127.0.0.1", 8080))?
    // Run the server, which will create a thread pool to handle incoming requests
    .run()
    .await
}
