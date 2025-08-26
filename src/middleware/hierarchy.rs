/// Hierarchical authorization middleware for role-based access control
use crate::{
    constants::{error_messages, roles},
    error::AppError,
    middleware::auth::AuthenticatedUser,
    utils::authorization,
};
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

/// Middleware that enforces hierarchical authorization rules
#[derive(Debug, Clone)]
pub struct HierarchyMiddleware {
    pub required_role: String,
}

impl HierarchyMiddleware {
    pub fn new(required_role: &str) -> Self {
        Self {
            required_role: required_role.to_string(),
        }
    }

    /// Create middleware that requires superadmin access
    pub fn superadmin() -> Self {
        Self::new(roles::SUPERADMIN)
    }

    /// Create middleware that requires admin access (admin or superadmin)
    pub fn admin() -> Self {
        Self::new(roles::ADMIN)
    }

    /// Create middleware that requires contributor access (contributor or above)
    pub fn contributor() -> Self {
        Self::new(roles::CONTRIBUTOR)
    }

    /// Create middleware that requires user access (any authenticated user)
    pub fn user() -> Self {
        Self::new(roles::USER)
    }
}

impl<S, B> Transform<S, ServiceRequest> for HierarchyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HierarchyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HierarchyMiddlewareService {
            service: Rc::new(service),
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct HierarchyMiddlewareService<S> {
    service: Rc<S>,
    required_role: String,
}

impl<S, B> Service<ServiceRequest> for HierarchyMiddlewareService<S>
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
        let service = Rc::clone(&self.service);
        let required_role = self.required_role.clone();

        Box::pin(async move {
            // Get the authenticated user from request extensions
            let user = req.extensions().get::<AuthenticatedUser>().cloned();

            match user {
                Some(user) => {
                    // Check if user has required role level
                    if authorization::has_minimum_role_level(&user.role, &required_role) {
                        service.call(req).await
                    } else {
                        Err(AppError::Forbidden("Access denied. Insufficient role level")
                        .into())
                    }
                }
                None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED).into()),
            }
        })
    }
}

/// Request extractor that ensures user has management permissions
#[derive(Debug, Clone)]
pub struct ManagerUser(pub AuthenticatedUser);

impl FromRequest for ManagerUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(match user {
            Some(user) if user.can_manage_users() => Ok(ManagerUser(user)),
            Some(_) => Err(AppError::Forbidden(
                "Access denied. User management privileges required",
            )),
            None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)),
        })
    }
}

/// Request extractor for translation management
#[derive(Debug, Clone)]
pub struct TranslationManager(pub AuthenticatedUser);

impl FromRequest for TranslationManager {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(match user {
            Some(user) if matches!(user.role.as_str(), roles::SUPERADMIN | roles::ADMIN | roles::CONTRIBUTOR) => {
                Ok(TranslationManager(user))
            },
            Some(_) => Err(AppError::Forbidden(
                "Access denied. Translation management privileges required",
            )),
            None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)),
        })
    }
}

/// Helper function to check if user can access a specific user resource
pub fn check_user_access(
    current_user: &AuthenticatedUser,
    target_user_id: Uuid,
    target_role: Option<&str>,
) -> Result<(), AppError> {
    if !current_user.can_access_user(target_user_id, target_role) {
        return Err(AppError::Forbidden(
            "Access denied. Insufficient permissions to access this user",
        ));
    }
    Ok(())
}

/// Helper function to check if user can manage a specific user
pub fn check_user_management_access(
    current_user: &AuthenticatedUser,
    target_role: &str,
) -> Result<(), AppError> {
    if !current_user.can_manage_user(target_role) {
        return Err(AppError::Forbidden(
            "Access denied. Cannot manage users with specified role"
        ));
    }
    Ok(())
}

/// Helper function to check translation ownership for modification
pub fn check_translation_modification_access(
    current_user: &AuthenticatedUser,
    translation_owner: Option<Uuid>,
) -> Result<(), AppError> {
    if !current_user.can_modify_translation(translation_owner) {
        return Err(AppError::Forbidden(
            "Access denied. You can only modify your own translations",
        ));
    }
    Ok(())
}
