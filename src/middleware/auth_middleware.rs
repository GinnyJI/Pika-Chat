use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse, body::BoxBody,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::rc::Rc;

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

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(header_str) = auth_header.to_str() {
                if header_str.starts_with("Bearer ") {
                    let token = &header_str[7..];
                    let decoding_key = DecodingKey::from_secret("secret_key_for_jwt".as_ref());

                    if decode::<serde_json::Value>(
                        token,
                        &decoding_key,
                        &Validation::new(Algorithm::HS256),
                    )
                    .is_ok()
                    {
                        return Box::pin(async move { service.call(req).await });
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
