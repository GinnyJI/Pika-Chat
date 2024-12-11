use crate::models::presence::{GetRoomPresence, UserPresence};
use crate::models::response::{ErrorResponse, MessageResponse};
use crate::websockets::chat_session::{ChatSession, RoomServer};
use actix::Addr;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RoomInfo {
    pub room_name: String,
}

#[derive(Serialize, ToSchema)]
pub struct Room {
    pub room_id: i64,
    pub room_name: String,
    pub user_id: i64, // The owner's user ID
}

#[derive(Serialize, ToSchema)]
pub struct RoomsResponse {
    pub req_user_id: i64, // The ID of the current user making the request
    pub rooms: Vec<Room>, // List of rooms with each room's owner's user_id
}

#[derive(Serialize, ToSchema)]
pub struct RoomMember {
    pub user_id: i64,
    pub username: String,
    pub avatar_url: Option<String>, // Include avatar URL
}

#[utoipa::path(
    get,
    path = "/api/rooms",
    responses(
        (status = 200, description = "List of rooms with user ID", body = RoomsResponse),
        (status = 401, description = "User ID missing in token", body = ErrorResponse),
        (status = 500, description = "Failed to retrieve rooms", body = ErrorResponse)
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    )
)]
pub async fn get_rooms(pool: web::Data<SqlitePool>, req: HttpRequest) -> impl Responder {
    if let Some(user_id) = req.extensions().get::<i64>() {
        match sqlx::query_as!(
            Room,
            "SELECT room_id as `room_id: i64`, room_name, user_id as `user_id: i64` FROM rooms"
        )
        .fetch_all(pool.get_ref())
        .await
        {
            Ok(rooms) => {
                info!("Retrieved {} rooms from the database", rooms.len());
                for room in &rooms {
                    info!(
                        "Room ID: {}, Room Name: {}, User ID: {}",
                        room.room_id, room.room_name, room.user_id
                    );
                }
                HttpResponse::Ok().json(RoomsResponse {
                    req_user_id: *user_id,
                    rooms,
                })
            }
            Err(e) => {
                info!("Failed to retrieve rooms: {}", e);
                // A failure to retrieve rooms typically indicates a server-side problem, such as a database connectivity issue.
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to retrieve rooms".into(),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "User ID missing in token".into(),
        })
    }
}

#[utoipa::path(
    post,
    path = "/api/rooms",
    request_body = RoomInfo,
    responses(
        (status = 201, description = "Room created successfully", body = Room),
        (status = 400, description = "Error creating room", body = ErrorResponse),
        (status = 401, description = "User ID missing in token", body = ErrorResponse)
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    )
)]
pub async fn create_room(
    pool: web::Data<SqlitePool>,
    room_info: web::Json<RoomInfo>,
    req: HttpRequest,
) -> impl Responder {
    info!("Before Starting create_room function");
    if let Some(user_id) = req.extensions().get::<i64>() {
        match sqlx::query!(
            "INSERT INTO rooms (room_name, user_id) VALUES (?, ?)",
            room_info.room_name,
            user_id
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(result) => {
                info!(
                    "Room '{}' created successfully by user '{}'",
                    room_info.room_name, user_id
                );
                HttpResponse::Created().json(Room {
                    room_id: result.last_insert_rowid(),
                    room_name: room_info.room_name.clone(),
                    user_id: *user_id,
                })
            }
            Err(e) => {
                info!("Failed to create room: {}", e);
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Error creating room".into(),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "User ID missing in token".into(),
        })
    }
}

#[utoipa::path(
    get,
    path = "/api/rooms/{room_id}/members",
    params(
        ("room_id" = i64, Path, description = "ID of the room"),
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    ),
    responses(
        (status = 200, description = "List of room members", body = [RoomMember]),
        (status = 404, description = "Room not found", body = ErrorResponse),
        (status = 500, description = "Failed to retrieve room members", body = ErrorResponse)
    )
)]

pub async fn get_room_members(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    _req: HttpRequest,
) -> impl Responder {
    let room_id = path.into_inner();

    // Check if the room exists
    let room_exists = sqlx::query!(
        "SELECT COUNT(*) as count FROM rooms WHERE room_id = ?",
        room_id
    )
    .fetch_one(pool.get_ref())
    .await
    .map(|row| row.count > 0)
    .unwrap_or(false);

    if !room_exists {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: "Room not found".into(),
        });
    }

    // Fetch members of the room with avatar_url
    match sqlx::query_as!(
        RoomMember,
        "SELECT u.user_id, u.username, u.avatar_url FROM users u \
        INNER JOIN user_rooms ur ON u.user_id = ur.user_id \
        WHERE ur.room_id = ?",
        room_id
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(e) => {
            error!("Failed to retrieve room members: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse { error: "Failed to retrieve room members".into() })
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/rooms/{room_id}/members",
    params(
        ("room_id" = i64, Path, description = "Room ID to add the user to"),
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    ),
    responses(
        (status = 200, description = "User added to the room successfully", body = MessageResponse),
        (status = 400, description = "Bad request: Error adding user to room", body = ErrorResponse),
        (status = 404, description = "Not Found: Room does not exist", body = ErrorResponse),
        (status = 401, description = "Unauthorized: User ID missing in token", body = ErrorResponse)
    )
)]
pub async fn add_room_member(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    req: HttpRequest,
) -> impl Responder {
    let room_id = path.into_inner();

    if let Some(user_id) = req.extensions().get::<i64>() {
        let room_exists = sqlx::query!(
            "SELECT 1 AS exists_flag FROM rooms WHERE room_id = ?",
            room_id
        )
        .fetch_optional(pool.get_ref())
        .await
        .map(|row| row.is_some())
        .unwrap_or(false);

        if !room_exists {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "Room does not exist".into(),
            });
        }

        match sqlx::query!(
            "INSERT INTO user_rooms (user_id, room_id) VALUES (?, ?)",
            user_id,
            room_id
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {
                info!("User '{}' added to room '{}'", user_id, room_id);
                HttpResponse::Ok().json(MessageResponse {
                    message: "User added to the room successfully".into(),
                })
            }
            Err(e) => {
                info!("Failed to add user to room: {}", e);
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Error adding user to room".into(),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "User ID missing in token".into(),
        })
    }
}

#[derive(Deserialize)]
struct QueryParams {
    user_id: i64,
}

#[utoipa::path(
    get,
    path = "/ws/rooms/{room_id}",
    params(
        ("room_id" = i64, Path, description = "Room ID to join via WebSocket"),
        ("user_id" = i64, Query, description = "User ID for authentication")
    ),
    responses(
        (status = 101, description = "Switching Protocols to WebSocket"),
        (status = 401, description = "Unauthorized: User ID missing or invalid", body = ErrorResponse),
        (status = 404, description = "Not Found: Room does not exist or user is not a member", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse),
    )
)]
pub async fn join_room_ws(
    req: HttpRequest,
    stream: web::Payload,
    room_id: web::Path<i64>,
    room_server: web::Data<Addr<RoomServer>>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, actix_web::Error> {
    let room_id = room_id.into_inner();
    info!("Attempting to join room with ID: {}", room_id);

    // Extract user_id from query parameters
    let query_params = match web::Query::<QueryParams>::from_query(req.query_string()) {
        Ok(params) => params,
        Err(_) => {
            error!("user_id query parameter is missing or invalid");
            return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Missing or invalid user_id".to_string(),
            }));
        }
    };
    let user_id = query_params.user_id;

    // Check if the room exists
    let room_exists = match sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM rooms WHERE room_id = ?)",
        room_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(exists) => exists != 0,
        Err(e) => {
            error!("Database error checking if room exists: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database error".to_string(),
            }));
        }
    };

    if !room_exists {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "Room does not exist".to_string(),
        }));
    }

    // Check if the user is a member of the room
    let is_member = match sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM user_rooms WHERE room_id = ? AND user_id = ?)",
        room_id,
        user_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(member) => member != 0,
        Err(e) => {
            error!(
                "Database error checking if user is a member of the room: {}",
                e
            );
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database error".to_string(),
            }));
        }
    };

    if !is_member {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "User is not a member of the room".to_string(),
        }));
    }

    // Start WebSocket session
    info!(
        "Starting WebSocket session for user {} in room {}",
        user_id, room_id
    );
    let session = ChatSession::new(room_id, user_id, room_server.get_ref().clone());
    ws::start(session, &req, stream)
}

#[utoipa::path(
    get,
    path = "/users/presence",
    responses(
        (status = 200, description = "Returns user presence statuses in the room", body = [UserPresence]),
        (status = 404, description = "Room not found or no users present", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    params(
        ("room_id" = i64, Path, description = "Room ID to retrieve presence info"),
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    )
)]
pub async fn get_user_presence(
    room_id: web::Path<i64>, // Directly use i64 instead of RoomId
    room_server: web::Data<Addr<RoomServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    // Send the GetRoomPresence message to the RoomServer actor to fetch presence data.
    match room_server
        .send(GetRoomPresence { room_id: *room_id })
        .await
    {
        Ok(presence) if !presence.is_empty() => {
            Ok(HttpResponse::Ok().json(presence)) // Return the list of users' presence statuses
        }
        Ok(_) => {
            // Return a 404 response if the room was found, but no users are present
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "No users found in the specified room.".to_string(),
            }))
        }
        Err(_) => {
            // Return a 500 response if there was an error in processing the request
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to retrieve user presence information.".to_string(),
            }))
        }
    }
}
