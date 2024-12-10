use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// Message-related models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub room_id: i64,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Fetch messages for a specific chatroom
pub async fn get_messages(token: &str, room_id: i64) -> Result<MessagesResponse, String> {
    let response = Request::get(&format!(
        "http://127.0.0.1:8080/api/rooms/{}/messages",
        room_id
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await
    .map_err(|_| "Failed to connect to the server".to_string())?;

    if response.ok() {
        response.json::<MessagesResponse>()
            .await
            .map_err(|_| "Failed to parse server response".to_string())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}

// Send a new message to a specific chatroom
pub async fn send_message(token: &str, room_id: i64, content: &str) -> Result<(), String> {
    let request_body = SendMessageRequest {
        content: content.to_string(),
    };

    let response = Request::post(&format!(
        "http://127.0.0.1:8080/api/rooms/{}/messages",
        room_id
    ))
    .header("Authorization", &format!("Bearer {}", token))
    .header("Content-Type", "application/json")
    .json(&request_body)
    .map_err(|_| "Failed to serialize request".to_string())?
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
