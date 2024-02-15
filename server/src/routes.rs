use actix_web::web;

use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/1")
            .service(web::scope("/login").route("", web::post().to(handlers::user::login)))
            .service(
                web::scope("/schools")
                    .route("", web::get().to(handlers::schools::get_all))
                    .route("", web::post().to(handlers::schools::create))
                    .route("/{id}", web::put().to(handlers::schools::update))
                    .route("/{id}", web::delete().to(handlers::schools::delete))
                    .service(
                        web::scope("/{school_id}/streams")
                            .route("", web::get().to(handlers::streams::get_all))
                            .route("", web::post().to(handlers::streams::create))
                            .route("/{id}", web::put().to(handlers::streams::update))
                            .route("/{id}", web::delete().to(handlers::streams::delete)),
                    )
                    .service(
                        web::scope("/{school_id}/years")
                            .route("", web::get().to(handlers::years::get_all))
                            .route("", web::post().to(handlers::years::create))
                            .route("/{id}", web::put().to(handlers::years::update))
                            .route("/{id}", web::delete().to(handlers::years::delete)),
                    )
                    .service(
                        web::scope("/{school_id}/groups")
                            .route("", web::get().to(handlers::groups::get_all))
                            .route("", web::post().to(handlers::groups::create)) // .route("/{id}", web::put().to(handlers::years::update))
                            .route("/{id}", web::delete().to(handlers::groups::delete)),
                    ),
            )
            .service(
                web::scope("/persons")
                    .route("/{id}", web::put().to(handlers::update_person))
                    .route("", web::post().to(handlers::add_person))
                    .route("", web::get().to(handlers::all_persons)),
            )
            .service(web::scope("/test").route("", web::get().to(handlers::test))),
    );
}
