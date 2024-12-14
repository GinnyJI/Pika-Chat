use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, PartialEq)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub avatar_url: Option<String>, // Optional field for avatar URL, only used by register
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn login(credentials: &Credentials) -> Result<LoginResponse, String> {
    let response = Request::post("http://127.0.0.1:8080/api/login")
        .header("Content-Type", "application/json")
        .json(credentials)
        .map_err(|_| "Failed to serialize request".to_string())?
        .send()
        .await
        .map_err(|_| "Failed to connect to the server".to_string())?;

    if (200..300).contains(&response.status()) {
        response.json::<LoginResponse>()
            .await
            .map_err(|_| "Invalid server response".to_string())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}

pub async fn register(credentials: &Credentials) -> Result<(), String> {
    let response = Request::post("http://127.0.0.1:8080/api/register")
        .header("Content-Type", "application/json")
        .json(credentials)
        .map_err(|_| "Failed to serialize request".to_string())?
        .send()
        .await;

    if let Ok(response) = response {
        if response.ok() {
            Ok(())
        } else {
            let err: ErrorResponse = response.json::<ErrorResponse>()
                .await
                .map_err(|_| "Invalid error response from server".to_string())?;
            Err(err.error)
        }
    } else {
        Err("Failed to connect to the server.".to_string())
    }
}

pub async fn logout(token: &str) -> Result<(), String> {
    let response = Request::post("http://127.0.0.1:8080/api/logout")
        // Include the Bearer token in the Authorization header
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| "Failed to connect to the server".to_string())?;

    if (200..300).contains(&response.status()) {
        Ok(())
    } else {
        let err: ErrorResponse = response.json::<ErrorResponse>()
            .await
            .map_err(|_| "Invalid error response from server".to_string())?;
        Err(err.error)
    }
}
