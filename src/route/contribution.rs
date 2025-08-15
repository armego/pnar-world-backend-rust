use actix_web::web;

use crate::handlers::contribution;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contributions")
            .route("", web::post().to(contribution::create_contribution))
            .route("", web::get().to(contribution::list_contributions))
            .route("/{id}", web::get().to(contribution::get_contribution))
            .route("/{id}", web::put().to(contribution::update_contribution))
            .route("/{id}", web::delete().to(contribution::delete_contribution))
    );
}
