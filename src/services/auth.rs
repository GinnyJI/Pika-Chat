use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, PartialEq)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(credentials: &Credentials) -> Result<LoginResponse, String> {
    Request::post("http://127.0.0.1:8080/api/login")
        .header("Content-Type", "application/json")
        .json(credentials)
        .map_err(|_| "Failed to serialize request".to_string())?
        .send()
        .await
        .map_err(|_| "Failed to connect to the server".to_string())?
        .json::<LoginResponse>()
        .await
        .map_err(|_| "Invalid server response".to_string())
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
            Err("Username already exists.".to_string())
        }
    } else {
        Err("Failed to connect to the server.".to_string())
    }
}
