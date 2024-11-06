use actix_web::{HttpResponse, Responder, web, HttpRequest, HttpMessage};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use log::info;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RoomInfo {
    pub room_name: String,
}

#[derive(Serialize, ToSchema)]
pub struct Room {
    pub room_id: i64,
    pub room_name: String,
    pub user_id: i64,
}

#[utoipa::path(
    post,
    path = "/api/rooms",
    request_body = RoomInfo, // Define RoomInfo as the request body schema
    responses(
        (status = 201, description = "Room created successfully", body = Room),
        (status = 400, description = "Error creating room"),
        (status = 401, description = "User ID not found")
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    )
)]
pub async fn create_room(
    pool: web::Data<SqlitePool>,
    room_info: web::Json<RoomInfo>,
    req: HttpRequest, // Used to extract user ID from JWT
) -> impl Responder {
    info!("Before Starting create_room function");
    if let Some(user_id) = req.extensions().get::<i64>() {
        match sqlx::query!(
            "INSERT INTO rooms (room_name, user_id) VALUES (?, ?)",
            room_info.room_name, user_id
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(result) => {
                info!("Room '{}' created successfully by user '{}'", room_info.room_name, user_id);
                HttpResponse::Created().json(Room {
                    room_id: result.last_insert_rowid(),
                    room_name: room_info.room_name.clone(),
                    user_id: *user_id,
                })
            }
            Err(e) => {
                info!("Failed to create room: {}", e);
                HttpResponse::BadRequest().body("Error creating room")
            }
        }
    } else {
        HttpResponse::Unauthorized().body("User ID not found")
    }
}

#[utoipa::path(
    post,
    path = "/api/rooms/{room_id}/join",
    params(
        ("room_id" = i64, Path, description = "Room ID to join"),
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    ),
    responses(
        (status = 200, description = "Joined room successfully"),
        (status = 400, description = "Bad request: Room does not exist"),
        (status = 400, description = "Bad request: Error joining room"),
        (status = 401, description = "User ID not found")
    )
)]
pub async fn join_room(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>, // Changed to i64
    req: HttpRequest, // Used to extract user ID from JWT
) -> impl Responder {
    let room_id = path.into_inner();

    if let Some(user_id) = req.extensions().get::<i64>() {
        // Check if the room exists in the `rooms` table
        let room_exists = sqlx::query!(
            "SELECT 1 AS exists_flag FROM rooms WHERE room_id = ?",
            room_id
        )
        .fetch_optional(pool.get_ref())
        .await
        .map(|row| row.is_some())
        .unwrap_or(false);

        if !room_exists {
            return HttpResponse::BadRequest().body("Room does not exist");
        }

        // If both user and room exist, proceed to add the user to the room
        match sqlx::query!(
            "INSERT INTO user_rooms (user_id, room_id) VALUES (?, ?)",
            user_id, room_id
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {
                info!("User '{}' joined room '{}'", user_id, room_id);
                HttpResponse::Ok().body("Joined room successfully")
            }
            Err(e) => {
                info!("Failed to join room: {}", e);
                HttpResponse::BadRequest().body("Error joining room")
            }
        }
    } else {
        HttpResponse::Unauthorized().body("User ID not found")
    }
}

#[utoipa::path(
    get,
    path = "/api/rooms",
    responses(
        (status = 200, description = "List of rooms", body = [Room]),
        (status = 500, description = "Failed to retrieve rooms")
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    )
)]
pub async fn get_rooms(pool: web::Data<SqlitePool>) -> impl Responder {
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
                info!("Room ID: {}, Room Name: {}, User ID: {}", room.room_id, room.room_name, room.user_id);
            }
            HttpResponse::Ok().json(rooms)
        }
        Err(e) => {
            info!("Failed to retrieve rooms: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve rooms")
        }
    }
}
