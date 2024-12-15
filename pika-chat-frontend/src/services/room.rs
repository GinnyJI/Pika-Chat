use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// Room related models
#[derive(Serialize, Clone, PartialEq)]
pub struct RoomInfo {
    pub room_name: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    pub room_id: i64,
    pub room_name: String,
    pub user_id: i64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RoomsResponse {
    pub req_user_id: i64,
    pub rooms: Vec<Room>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RoomMember {
    pub user_id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Service functions for room-related backend interactions
pub async fn get_rooms(token: &str) -> Result<RoomsResponse, String> {
    let response = Request::get("http://127.0.0.1:8080/api/rooms")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| "Failed to connect to the server".to_string())?;

    if response.ok() {
        response.json::<RoomsResponse>()
            .await
            .map_err(|_| "Failed to parse server response".to_string())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}

pub async fn create_room(token: &str, room_info: &RoomInfo) -> Result<Room, String> {
    let response = Request::post("http://127.0.0.1:8080/api/rooms")
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(room_info)
        .map_err(|_| "Failed to serialize request".to_string())?
        .send()
        .await
        .map_err(|_| "Failed to connect to the server".to_string())?;

    if response.status() == 201 {
        response.json::<Room>()
            .await
            .map_err(|_| "Failed to parse server response".to_string())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}

pub async fn get_room_members(token: &str, room_id: i64) -> Result<Vec<RoomMember>, String> {
    let response = Request::get(&format!(
        "http://127.0.0.1:8080/api/rooms/{}/members",
        room_id
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await
    .map_err(|_| "Failed to connect to the server".to_string())?;

    if response.ok() {
        response.json::<Vec<RoomMember>>()
            .await
            .map_err(|_| "Failed to parse server response".to_string())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}

#[allow(dead_code)]
pub async fn add_room_member(token: &str, room_id: i64) -> Result<(), String> {
    let response = Request::post(&format!(
        "http://127.0.0.1:8080/api/rooms/{}/members",
        room_id
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await
    .map_err(|_| "Failed to connect to the server".to_string())?;

    if response.ok() {
        Ok(())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserPresence {
    pub user_id: i64,
    pub is_online: bool,
}
pub async fn get_user_presence(token: &str, room_id: i64) -> Result<Vec<UserPresence>, String> {
    let response = Request::get(&format!(
        "http://127.0.0.1:8080/api/users/presence/{}",
        room_id
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await
    .map_err(|_| "Failed to connect to the server".to_string())?;

    if response.ok() {
        // Log the successful request
        println!("Successfully fetched presence data for room ID: {}", room_id);

        response
            .json::<Vec<UserPresence>>()
            .await
            .map_err(|_| "Failed to parse server response".to_string())
    } else if response.status() == 404 {
        // Log the specific error case
        println!("Room ID {} not found or no users present.", room_id);
        Err("Room not found or no users present".to_string())
    } else {
        let err: ErrorResponse = response
            .json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;

        // Log the generic error
        println!("Error while fetching presence for room ID {}: {}", room_id, err.error);
        Err(err.error)
    }
}
