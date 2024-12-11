use serde::{Deserialize, Serialize};

pub type RoomId = i64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub room_id: RoomId,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageRequest {
    pub content: String,
}

impl Message {
    // This method might be used for sending messages over WebSocket
    pub fn new(room_id: RoomId, message: String) -> Self {
        Message { room_id, message}
    }
}
