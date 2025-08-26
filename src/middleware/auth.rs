use crate::{constants::{error_messages, roles}, error::AppError, utils::jwt};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use sqlx::{PgPool, Row};
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

impl AuthenticatedUser {
    /// Check if the user has superadmin role (highest level access)
    pub fn is_superadmin(&self) -> bool {
        self.role == roles::SUPERADMIN
    }

    /// Check if the user has admin role or higher
    pub fn is_admin(&self) -> bool {
        self.is_superadmin() || self.role == roles::ADMIN
    }

    /// Check if the user has moderator role or higher
    pub fn is_moderator(&self) -> bool {
        self.is_admin() || self.role == roles::MODERATOR
    }

    /// Check if the user has contributor role or higher
    pub fn is_contributor(&self) -> bool {
        self.is_moderator() || self.role == roles::CONTRIBUTOR
    }

    /// Check if the user can access another user's data based on hierarchy
    /// - Superadmin: can access all users
    /// - Admin: can access users of same rank and below
    /// - Others: can only access their own data
    pub fn can_access_user(&self, target_user_id: Uuid, target_role: Option<&str>) -> bool {
        use crate::utils::authorization::{can_view_user};
        
        match target_role {
            Some(role) => can_view_user(&self.role, self.user_id, role, target_user_id),
            None => self.is_admin() || self.user_id == target_user_id,
        }
    }

    /// Check if the user can manage another user based on role hierarchy
    pub fn can_manage_user(&self, target_role: &str) -> bool {
        use crate::utils::authorization::can_manage_user;
        can_manage_user(&self.role, target_role)
    }

    /// Check if the user can modify dictionary entries
    /// Contributors and above can modify entries
    pub fn can_modify_dictionary(&self) -> bool {
        self.is_contributor()
    }

    /// Check if the user can verify dictionary entries
    /// Moderators and above can verify entries
    pub fn can_verify_dictionary(&self) -> bool {
        self.is_moderator()
    }

    /// Check if the user can modify translations based on ownership
    pub fn can_modify_translation(&self, translation_owner: Option<Uuid>) -> bool {
        use crate::utils::authorization::can_modify_translation;
        can_modify_translation(&self.role, self.user_id, translation_owner)
    }

    /// Check if the user can delete translations based on ownership
    pub fn can_delete_translation(&self, translation_owner: Option<Uuid>) -> bool {
        use crate::utils::authorization::can_delete_translation;
        can_delete_translation(&self.role, self.user_id, translation_owner)
    }

    /// Check if the user can review translations
    /// Contributors and above can review translations
    pub fn can_review_translations(&self) -> bool {
        self.is_contributor()
    }

    /// Check if the user can review contributions
    /// Moderators and above can review contributions
    pub fn can_review_contributions(&self) -> bool {
        self.is_moderator()
    }

    /// Check if the user can access analytics data
    /// Moderators and above can access analytics
    pub fn can_access_analytics(&self) -> bool {
        self.is_moderator()
    }

    /// Check if the user can manage other users
    /// Only superadmin and admin can manage users
    pub fn can_manage_users(&self) -> bool {
        use crate::utils::authorization::can_access_user_management;
        can_access_user_management(&self.role)
    }

    /// Check if the user can delete any content
    /// Admins and above can delete any content
    pub fn can_delete_any_content(&self) -> bool {
        self.is_admin()
    }

    /// Get role hierarchy level (higher number = more permissions)
    pub fn role_level(&self) -> u8 {
        match self.role.as_str() {
            roles::SUPERADMIN => 5,
            roles::ADMIN => 4,
            roles::MODERATOR => 3,
            roles::CONTRIBUTOR => 2,
            roles::USER => 1,
            _ => 0, // Unknown role gets lowest access
        }
    }

    /// Check if user has at least the specified role level
    pub fn has_role_level(&self, required_level: u8) -> bool {
        self.role_level() >= required_level
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(user.ok_or_else(|| AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)))
    }
}

// Role-based extractors for different access levels

#[derive(Debug, Clone)]
pub struct SuperAdminUser(pub AuthenticatedUser);

impl FromRequest for SuperAdminUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(match user {
            Some(user) if user.is_superadmin() => Ok(SuperAdminUser(user)),
            Some(_) => Err(AppError::Forbidden(error_messages::SUPERADMIN_ACCESS_REQUIRED)),
            None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AdminUser(pub AuthenticatedUser);

impl FromRequest for AdminUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(match user {
            Some(user) if user.is_admin() => Ok(AdminUser(user)),
            Some(_) => Err(AppError::Forbidden(error_messages::ADMIN_ACCESS_REQUIRED)),
            None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModeratorUser(pub AuthenticatedUser);

impl FromRequest for ModeratorUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(match user {
            Some(user) if user.is_moderator() => Ok(ModeratorUser(user)),
            Some(_) => Err(AppError::Forbidden(error_messages::MODERATOR_ACCESS_REQUIRED)),
            None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ContributorUser(pub AuthenticatedUser);

impl FromRequest for ContributorUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let user = extensions.get::<AuthenticatedUser>().cloned();

        ready(match user {
            Some(user) if user.is_contributor() => Ok(ContributorUser(user)),
            Some(_) => Err(AppError::Forbidden(error_messages::CONTRIBUTOR_ACCESS_REQUIRED)),
            None => Err(AppError::Unauthorized(error_messages::USER_NOT_AUTHENTICATED)),
        })
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
                if auth_str.starts_with("Bearer ") {
                    Some(auth_str[7..].to_string())
                } else {
                    None
                }
            });

        let service = Rc::clone(&self.service);

        Box::pin(async move {
            if let Some(token) = token {
                tracing::debug!("Token found: {}", &token[..std::cmp::min(20, token.len())]);
                match jwt::verify_token(&token) {
                    Ok(claims) => {
                        let user_id = claims.user_id()?;
                        
                        // Get the database pool from app data
                        let pool = req.app_data::<web::Data<PgPool>>()
                            .ok_or_else(|| AppError::Internal("Database pool not found".to_string()))?;

                        // Fetch user role from database
                        let user_role = match sqlx::query("SELECT role FROM users WHERE id = $1")
                            .bind(user_id)
                            .fetch_optional(pool.get_ref())
                            .await
                        {
                            Ok(Some(row)) => row.get::<String, _>("role"),
                            Ok(None) => return Err(AppError::Unauthorized(error_messages::USER_NOT_FOUND).into()),
                            Err(_) => "user".to_string(), // Fallback to default role if DB query fails
                        };

                        let user = AuthenticatedUser {
                            user_id,
                            role: user_role,
                        };
                        req.extensions_mut().insert(user);
                        service.call(req).await
                    }
                    Err(err) => {
                        tracing::warn!("JWT verification failed: {}", err);
                        Err(err.into())
                    }
                }
            } else {
                tracing::debug!("No token found in request");
                Err(AppError::Unauthorized(error_messages::MISSING_AUTH_TOKEN).into())
            }
        })
    }
}
