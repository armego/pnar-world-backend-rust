use actix_web::web;

use crate::handlers::translation;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/translations")
            .route("", web::post().to(translation::create_translation))
            .route("", web::get().to(translation::list_translations))
            .route("/{id}", web::get().to(translation::get_translation))
            .route("/{id}", web::put().to(translation::update_translation))
            .route("/{id}", web::delete().to(translation::delete_translation))
    );
}
