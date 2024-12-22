use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // User ID
    pub username: String, // Username
    pub iat: usize,    // Issued at time
    pub exp: usize,    // Expiration time
}
