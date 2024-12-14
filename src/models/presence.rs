use actix::Message;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema; // Import ToSchema for OpenAPI schema generation
use crate::websockets::chat_session::{UserId, RoomId};

/// Message to request the current presence status of all users in a specific room
pub struct GetRoomPresence {
    pub room_id: RoomId,
}

impl Message for GetRoomPresence {
    type Result = Vec<UserPresence>;
}

/// Structure representing the presence status of a user
#[derive(Serialize, Deserialize, Debug, ToSchema)] // Derive ToSchema for OpenAPI support
pub struct UserPresence {
    pub user_id: UserId,
    pub is_online: bool,
}

impl UserPresence {
    pub fn new(user_id: UserId, is_online: bool) -> Self {
        UserPresence { user_id, is_online }
    }
}
