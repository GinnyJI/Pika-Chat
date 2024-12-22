use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize, ToSchema)]
pub struct TokenResponse {
    pub token: String,             // JWT token
    pub username: String,          // Username of the logged-in user
    pub avatar_url: Option<String>, // Optional avatar URL of the user
}
