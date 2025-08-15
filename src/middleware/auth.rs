use crate::{error::AppError, utils::jwt};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub role: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(user.ok_or_else(|| AppError::Unauthorized("User not authenticated".to_string())))
    }
}

#[derive(Debug, Clone)]
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_str| {
                println!("Auth header: {}", auth_str); // Debug log
                if auth_str.starts_with("Bearer ") {
                    Some(auth_str[7..].to_string())
                } else {
                    None
                }
            });

        let service = self.service.clone();

        Box::pin(async move {
            if let Some(token) = token {
                println!("Token found: {}", &token[..std::cmp::min(20, token.len())]); // Debug log
                match jwt::verify_token(&token) {
                    Ok(claims) => {
                        let user_id = claims.user_id()?;
                        let user = AuthenticatedUser {
                            user_id,
                            role: "user".to_string(), // TODO: fetch from database or JWT claims
                        };
                        req.extensions_mut().insert(user);
                        service.call(req).await
                    }
                    Err(err) => {
                        println!("JWT verification failed: {}", err); // Debug log
                        Err(err.into())
                    }
                }
            } else {
                println!("No token found in request"); // Debug log
                Err(AppError::Unauthorized("Missing authentication token".to_string()).into())
            }
        })
    }
}
