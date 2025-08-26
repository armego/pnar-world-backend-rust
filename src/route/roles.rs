use actix_web::web;
use crate::handlers::roles;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .service(roles::list_roles)
            .service(roles::list_assignable_roles)
            .service(roles::list_manageable_roles)
    );
}
