use actix_web::{HttpResponse, Responder};

// Define the route handler function
pub async fn test_protected_route() -> impl Responder {
    HttpResponse::Ok().body("This is a protected route")
}
