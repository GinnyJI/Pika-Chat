use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse, body::BoxBody, web::Data,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::rc::Rc;
use log::info;
use crate::models::claim::Claims; // Ensure Claims struct is imported
use sqlx::SqlitePool;
use crate::config::state::TOKEN_BLACKLIST;

// Define the AuthMiddleware struct
pub struct AuthMiddleware;

// Implement the Transform trait for AuthMiddleware
impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

// Define the middleware struct
pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

// Implement the Service trait for the middleware
impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let pool = req.app_data::<Data<SqlitePool>>().cloned();

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(header_str) = auth_header.to_str() {
                if header_str.starts_with("Bearer ") {
                    let token = &header_str[7..];
                    
                    // Check if the token is in the blacklist
                    {
                        let blacklist = TOKEN_BLACKLIST.lock().unwrap();
                        info!("blacklist: {:?}, token: {}", *blacklist, token);
                        if blacklist.contains(token) {
                            info!("Token found in blacklist, blocking access");
                            return Box::pin(async move {
                                let (req, _payload) = req.into_parts();
                                let response = HttpResponse::Unauthorized().body("Token is invalid");
                                Ok(ServiceResponse::new(req, response.map_into_boxed_body()))
                            });
                        }
                    }

                    let decoding_key = DecodingKey::from_secret("secret_key_for_jwt".as_ref());
                    if let Ok(decoded_token) = decode::<Claims>(
                        token,
                        &decoding_key,
                        &Validation::new(Algorithm::HS256),
                    ) {
                        // validation: check if the sub (user ID) exists in the database
                        if let Some(pool) = pool {
                            let user_id = decoded_token.claims.sub.parse::<i64>().ok();
                            if let Some(user_id) = user_id {
                                let future = async move {
                                    if let Ok(mut conn) = pool.acquire().await {
                                        let user_exists = sqlx::query!(
                                            "SELECT 1 AS exists_flag FROM users WHERE user_id = ?",
                                            user_id
                                        )
                                        .fetch_optional(&mut conn)
                                        .await
                                        .map(|row| row.is_some())
                                        .unwrap_or(false);

                                        if user_exists {
                                            info!(
                                                "Token validated successfully for user ID: {}, username: {}",
                                                decoded_token.claims.sub, decoded_token.claims.username
                                            );
                                            return service.call(req).await;
                                        }
                                    }

                                    info!("Token's user ID not found in the database.");
                                    let (req, _payload) = req.into_parts();
                                    let response = HttpResponse::Unauthorized().body("Invalid user");
                                    Ok(ServiceResponse::new(req, response.map_into_boxed_body()))
                                };
                                return Box::pin(future);
                            }
                        }
                    }
                }
            }
        }

        Box::pin(async move {
            let (req, _payload) = req.into_parts();
            let response = HttpResponse::Unauthorized().body("Unauthorized");
            Ok(ServiceResponse::new(req, response.map_into_boxed_body()))
        })
    }
}
