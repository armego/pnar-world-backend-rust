use actix_web::{HttpResponse, Result};

use crate::{
    constants::roles::get_all_roles,
    error::AppError,
};

/// Get all application roles (Public endpoint)
#[utoipa::path(
    get,
    path = "/api/v1/roles",
    tag = "roles",
    responses(
        (status = 200, description = "Roles retrieved successfully", body = [crate::constants::roles::RoleInfo]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_roles() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(get_all_roles()))
}
