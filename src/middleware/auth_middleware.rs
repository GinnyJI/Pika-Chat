use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse, body::BoxBody, web::Data,
}; // Import essential components for HTTP handling and request/response types
// Import future types for async operations in middleware
use futures_util::future::{ok, LocalBoxFuture, Ready};
// Import functions and structs for JWT decoding
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::rc::Rc;
use log::info;
// Ensure Claims struct is imported for token validation
use crate::models::claim::Claims;
use sqlx::SqlitePool;
// Import the global token blacklist for token revocation
use crate::config::state::TOKEN_BLACKLIST;

// Define the AuthMiddleware struct for implementing middleware behavior
pub struct AuthMiddleware;

// Implement the Transform trait for AuthMiddleware to allow it to modify the behavior of services
impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    // The type of middleware returned after transformation
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    // The future type that initializes the middleware
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // This function initializes the middleware and wraps the service
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

// Define the actual middleware struct that handles requests
pub struct AuthMiddlewareMiddleware<S> {
    // Using Rc (Reference Counted) to allow multiple parts of the code to share ownership of the service.
    // This ensures that the service instance can be accessed by multiple asynchronous tasks without 
    // transferring ownership, which maintains shared access while keeping memory management safe.
    // Rc automatically manages the reference count and deallocates the service when no references remain.
    service: Rc<S>,
}

// Implement the Service trait for AuthMiddlewareMiddleware to process each request
impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // This method checks if the underlying service is ready to process requests
    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // Forward the readiness check to the underlying service to ensure it is ready to handle requests.
        // This allows the middleware to rely on the inner service's readiness state, maintaining consistent
        // behavior and avoiding redundant checks. If the inner service is not ready, this method will return
        // that status, ensuring the middleware responds accordingly.
        self.service.poll_ready(cx)
    }

    // Main logic to handle incoming requests
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let pool = req.app_data::<Data<SqlitePool>>().cloned(); // Retrieve and clone the database pool if available

        // Check if the "Authorization" header is present
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(header_str) = auth_header.to_str() {
                // Check if the header starts with "Bearer "
                if header_str.starts_with("Bearer ") {
                    // Extract the token from the header
                    let token = &header_str[7..];

                    // Check if the token is in the blacklist for revocation
                    {
                        let blacklist = TOKEN_BLACKLIST.lock().unwrap();
                        info!("blacklist: {:?}, token: {}", *blacklist, token);
                        if blacklist.contains(token) {
                            info!("Token found in blacklist, blocking access");
                            // Return an unauthorized response if the token is in the blacklist
                            return Box::pin(async move {
                                let (req, _payload) = req.into_parts();
                                let response = HttpResponse::Unauthorized().body("Token is invalid");
                                Ok(ServiceResponse::new(req, response.map_into_boxed_body()))
                            });
                        }
                    }

                    // Decode the JWT token using a secret key for verification
                    let decoding_key = DecodingKey::from_secret("secret_key_for_jwt".as_ref());
                    if let Ok(decoded_token) = decode::<Claims>(
                        token,
                        &decoding_key,
                        &Validation::new(Algorithm::HS256),
                    ) {
                        // Check if the user ID (sub) in the token exists in the database
                        if let Some(pool) = pool {
                            let user_id = decoded_token.claims.sub.parse::<i64>().ok();
                            if let Some(user_id) = user_id {
                                let future = async move {
                                    // Query the database to check if the user ID exists
                                    if let Ok(mut conn) = pool.acquire().await {
                                        let user_exists = sqlx::query!(
                                            "SELECT 1 AS exists_flag FROM users WHERE user_id = ?",
                                            user_id
                                        )
                                        .fetch_optional(&mut conn)
                                        .await
                                        .map(|row| row.is_some())
                                        .unwrap_or(false);

                                        // If the user exists, proceed with the service call
                                        if user_exists {
                                            info!(
                                                "Token validated successfully for user ID: {}, username: {}",
                                                decoded_token.claims.sub, decoded_token.claims.username
                                            );
                                            return service.call(req).await;
                                        }
                                    }

                                    // Log and return an unauthorized response if the user ID is not found
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

        // Return an unauthorized response if no valid token is provided
        Box::pin(async move {
            let (req, _payload) = req.into_parts();
            let response = HttpResponse::Unauthorized().body("Unauthorized");
            Ok(ServiceResponse::new(req, response.map_into_boxed_body()))
        })
    }
}
