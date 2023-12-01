use actix_web::web;

use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/1").service(web::scope("/test").route("", web::get().to(handlers::test))),
    );
}
