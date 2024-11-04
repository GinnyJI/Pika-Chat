use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub room_id: i32,
    pub room_name: String,
    pub created_at: String,
}
