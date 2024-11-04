use actix_web::{HttpResponse, Responder, web, HttpRequest, HttpMessage};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use log::info;

#[derive(Deserialize)]
pub struct RoomInfo {
    pub room_name: String,
}

#[derive(Serialize)]
pub struct Room {
    pub room_id: Option<i64>, // Room ID as Option<i64>
    pub room_name: String,
    pub user_id: i32, // Owner's user ID
}

pub async fn create_room(
    pool: web::Data<SqlitePool>,
    room_info: web::Json<RoomInfo>,
    req: HttpRequest, // Used to extract user ID from JWT
) -> impl Responder {
    // Extract user ID from request extensions (set by middleware)
    if let Some(user_id) = req.extensions().get::<i32>() {
        match sqlx::query!(
            "INSERT INTO rooms (room_name, user_id) VALUES (?, ?)",
            room_info.room_name, user_id
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(result) => {
                info!("Room '{}' created successfully by user '{}'", room_info.room_name, user_id);
                HttpResponse::Created().json(serde_json::json!({ "room_id": result.last_insert_rowid() }))
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

pub async fn join_room(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
    req: HttpRequest, // Used to extract user ID from JWT
) -> impl Responder {
    // Extract the inner i32 value from the web::Path wrapper, which contains the room_id path parameter.
    // In a route like /rooms/{room_id}, the {room_id} part is captured as a path parameter and passed to the handler as web::Path.
    let room_id = path.into_inner();

    // Extract user ID from request extensions (set by middleware)
    if let Some(user_id) = req.extensions().get::<i32>() {

        // Check if the user exists in the `users` table
        let user_exists = sqlx::query!(
            "SELECT 1 AS exists_flag FROM users WHERE user_id = ?",
            user_id
        )
        .fetch_optional(pool.get_ref())
        .await
        .map(|row| row.is_some())
        .unwrap_or(false);

        if !user_exists {
            return HttpResponse::BadRequest().body("User does not exist");
        }

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

pub async fn get_rooms(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as!(
        Room,
        "SELECT room_id as `room_id: Option<i64>`, room_name, user_id as `user_id: i32` FROM rooms"
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rooms) => {
            info!("Retrieved {} rooms", rooms.len());
            HttpResponse::Ok().json(rooms)
        }
        Err(e) => {
            info!("Failed to retrieve rooms: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve rooms")
        }
    }
}
