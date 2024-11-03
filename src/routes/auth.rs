use actix_web::{post, HttpResponse, web, Responder, HttpRequest};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;
use sqlx::SqlitePool;
use log::{info, error};
use crate::models::claim::Claims; // Import the Claims struct
use serde::Deserialize; // Ensure this line is present
use crate::config::state::TOKEN_BLACKLIST;

#[derive(Deserialize)]
struct RegisterData {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/register")]
async fn register_user(
    pool: web::Data<SqlitePool>,
    user_data: web::Json<RegisterData>,
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
            HttpResponse::Created().body("User created successfully")
        }
        Err(e) => {
            error!("Failed to register user '{}': {:?}", user_data.username, e);
            HttpResponse::BadRequest().body("User could not be created")
        }
    }
}

#[post("/login")]
async fn login_user(
    pool: web::Data<SqlitePool>,
    login_data: web::Json<LoginData>,
) -> HttpResponse {
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
            return HttpResponse::Ok().json(serde_json::json!({ "token": token }));
        } else {
            info!("User '{}' failed to log in due to incorrect password.", login_data.username);
        }
    } else {
        info!("Login attempt failed: user '{}' not found.", login_data.username);
    }

    HttpResponse::Unauthorized().body("Invalid username or password")
}

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
                return HttpResponse::Ok().json("Logged out successfully");
            }
        }
    }
    HttpResponse::BadRequest().body("Invalid or missing token")
}
