use actix_web::{HttpResponse, Result, get};

use crate::{
    constants::roles::{get_all_roles, get_assignable_roles_info, get_manageable_roles_info},
    error::AppError,
    middleware::hierarchy::ManagerUser,
};

/// Get all application roles (Public endpoint)
)]
#[get("")]
pub async fn list_roles() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(get_all_roles()))
}

/// Get roles that can be assigned by the current user (User Management)
)]
#[get("/assignable")]
pub async fn list_assignable_roles(
    manager_user: ManagerUser, // Only users with management privileges
) -> Result<HttpResponse, AppError> {
    let assignable_roles = get_assignable_roles_info(&manager_user.0.role);
    Ok(HttpResponse::Ok().json(assignable_roles))
}

/// Get roles that can be managed by the current user (for filtering user lists)
)]
#[get("/manageable")]
pub async fn list_manageable_roles(
    manager_user: ManagerUser, // Only users with management privileges
) -> Result<HttpResponse, AppError> {
    let manageable_roles = get_manageable_roles_info(&manager_user.0.role);
    Ok(HttpResponse::Ok().json(manageable_roles))
}
