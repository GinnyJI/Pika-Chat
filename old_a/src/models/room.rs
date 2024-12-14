use serde::{Deserialize, Serialize};

// TODO: Check if necessary
#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub room_id: i32,
    pub room_name: String,
    pub created_at: String,
}
