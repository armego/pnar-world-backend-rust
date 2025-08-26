use actix_web::web;
use crate::handlers::book;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .service(book::list_books)
            .service(book::get_book)
            .service(book::create_book)
            .service(book::update_book)
            .service(book::delete_book)
            .service(book::get_my_books)
    );
}
