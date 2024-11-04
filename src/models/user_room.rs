use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRoom {
    pub user_id: i32,
    pub room_id: i32,
    pub joined_at: String,
}
