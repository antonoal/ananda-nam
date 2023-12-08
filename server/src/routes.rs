use actix_web::web;

use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/1")
            .service(
                web::scope("/persons")
                    .route("/{id}", web::put().to(handlers::update_person))
                    .route("", web::post().to(handlers::add_person))
                    .route("", web::get().to(handlers::all_persons)),
            )
            .service(web::scope("/test").route("", web::get().to(handlers::test))),
    );
}
