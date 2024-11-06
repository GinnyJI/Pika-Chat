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
    pub token: String,
}
