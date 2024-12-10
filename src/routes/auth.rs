use actix_web::{post, HttpResponse, web, Responder, HttpRequest};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;
use sqlx::SqlitePool;
use log::{info, error};
use crate::models::claim::Claims;
use serde::Deserialize;
use crate::config::state::TOKEN_BLACKLIST;
use utoipa::ToSchema;
use crate::models::response::{MessageResponse, ErrorResponse, TokenResponse};

#[derive(Deserialize, ToSchema)]
pub struct AuthData {
    username: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "/api/register",
    request_body = AuthData,
    responses(
        (status = 201, description = "User created successfully", body = MessageResponse),
        (status = 400, description = "User could not be created", body = ErrorResponse)
    )
)]
#[post("/register")]
async fn register_user(
    pool: web::Data<SqlitePool>,
    user_data: web::Json<AuthData>,
) -> HttpResponse {
    let hashed_password = hash(&user_data.password, DEFAULT_COST).unwrap();
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        user_data.username,
        hashed_password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            info!("User '{}' registered successfully.", user_data.username);
            HttpResponse::Created().json(MessageResponse { message: "User created successfully".into() })
        }
        Err(e) => {
            error!("Failed to register user '{}': {:?}", user_data.username, e);
            HttpResponse::BadRequest().json(ErrorResponse { error: "User could not be created".into() })
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = AuthData,
    responses(
        (status = 200, description = "User logged in successfully", body = TokenResponse),
        (status = 401, description = "Unauthorized: Invalid username or password or User ID missing in token", body = ErrorResponse),
        (status = 401, description = "Unauthorized: User ID missing in token", body = ErrorResponse)
    )
)]

#[post("/login")]
async fn login_user(
    pool: web::Data<SqlitePool>,
    login_data: web::Json<AuthData>,
) -> HttpResponse {
    // Fetch user from the database based on the provided username
    let user = sqlx::query!(
        "SELECT user_id, username, password_hash, created_at FROM users WHERE username = ?",
        login_data.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    if let Ok(Some(user)) = user {
        let is_valid = verify(&login_data.password, &user.password_hash).unwrap();

        if is_valid {
            let now = Utc::now().timestamp() as usize;
            let expiration = Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: user.user_id.expect("User ID should not be None").to_string(),  // Use user_id for sub
                username: user.username.clone(),  // Added username to claims
                iat: now,  // Issued at time
                exp: expiration,  // Expiration time
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret_key_for_jwt".as_ref()),
            ).unwrap();

            info!("User '{}' logged in successfully.", user.username);
            return HttpResponse::Ok().json(TokenResponse { token });
        } else {
            info!("User '{}' failed to log in due to incorrect password.", login_data.username);
            return HttpResponse::Unauthorized().json(ErrorResponse { error: "Unauthorized: Invalid username or password".into() });
        }
    } else {
        info!("Login attempt failed: user '{}' not found.", login_data.username);
        HttpResponse::Unauthorized().json(ErrorResponse { error: "Unauthorized: User ID missing in token".into() })
    }
}

#[utoipa::path(
    post,
    path = "/api/logout",
    responses(
        (status = 200, description = "Logged out successfully", body = MessageResponse),
        (status = 400, description = "Invalid or missing token", body = ErrorResponse)
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer <JWT Token>")
    )
)]
pub async fn logout_user(req: HttpRequest) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if header_str.starts_with("Bearer ") {
                let token = header_str[7..].to_string();
                {
                    let mut blacklist = TOKEN_BLACKLIST.lock().unwrap();
                    blacklist.insert(token.clone());
                    info!("Token '{}' added to blacklist. Current blacklist: {:?}", token, *blacklist);
                }
                return HttpResponse::Ok().json(MessageResponse { message: "Logged out successfully".into() });
            }
        }
    }
    HttpResponse::BadRequest().json(ErrorResponse { error: "Invalid or missing token".into() })
}
