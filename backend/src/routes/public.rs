use actix_web::{web};
use crate::handlers::public::{users, courses};

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/users")
            .route(web::get().to(users::get_users))
        )
    .service(
        web::resource("/courses")
            .route(web::get().to(courses::get_courses))
        );
}

