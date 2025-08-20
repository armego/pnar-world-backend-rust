use crate::handlers::alphabet;
use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/alphabets")
            .route("", web::get().to(alphabet::list_alphabets))
            .route("/convert", web::post().to(alphabet::convert_text)),
    );
}
