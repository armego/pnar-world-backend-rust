use actix_web::web;

use crate::handlers::analytics;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/analytics")
            .route("", web::post().to(analytics::create_analytics))
            .route("", web::get().to(analytics::list_analytics))
            .route("/anonymous", web::post().to(analytics::create_anonymous_analytics))
            .route("/{id}", web::get().to(analytics::get_analytics))
            .route("/{id}", web::put().to(analytics::update_analytics))
            .route("/{id}", web::delete().to(analytics::delete_analytics))
            .route("/words/{word_id}/stats", web::get().to(analytics::get_word_stats))
    );
}
